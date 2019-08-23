use clap;
use actix::prelude::*;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use prometheus::{opts, labels, Registry, Gauge, Encoder, TextEncoder};
use std::sync::Mutex;
use std::io::Result;
use bytes::Bytes;
use futures::stream::once;

mod thermometry;
use thermometry::{Thermometry, GetTemperature};

struct AppState {
    registry: Mutex<Registry>,
    t_gauge: Mutex<Gauge>,
    thermometry: Addr<Thermometry>,
}

pub fn start(_cli_args: clap::ArgMatches) -> Result<()> {
    let sys = System::new("thermomtry_app");

    let thermometry = Thermometry.start();
    let registry = Registry::new();
    let opts = opts!(
        "temp_celsius", "Hardware monitor for temperature (input)",
        labels!{"sensor" => "soc_chip_temp",}
    ).namespace("node").subsystem("hwmon");
    let t_gauge = Gauge::with_opts(opts).unwrap();
    registry.register(Box::new(t_gauge.clone())).unwrap();

    let app_state = web::Data::new(
        AppState {
            registry: Mutex::new(registry),
            t_gauge: Mutex::new(t_gauge),
            thermometry: thermometry,
        }
    );

    HttpServer::new(
        move || {
            App::new()
               .register_data(app_state.clone())
               .service(metrics)
        }
    )
    .bind("0.0.0.0:9090")?
    .start();

    sys.run()
}

#[get("/metrics")]
fn metrics(data: web::Data<AppState>) -> impl Responder {
    let registry = data.registry.lock().unwrap();
    let t_gauge = data.t_gauge.lock().unwrap().clone();

    // TODO: thermometry stream read into gauge
    let thermo_pipeline = data.thermometry
        .send(GetTemperature)
        .map(move |result| {
            println!("===>>> {:?}", result);
            let t = result.unwrap();
            t_gauge.set(t);
        })
        .map_err(|e| println!("Thermo pipeline error: {}", e));
    Arbiter::spawn(thermo_pipeline);

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metrics = registry.gather();
    encoder.encode(&metrics, &mut buffer).unwrap();
    
    let page = String::from_utf8(buffer).unwrap();
    HttpResponse::Ok().body(page)
}