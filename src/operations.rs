use clap;
use actix::prelude::*;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use prometheus::{opts, labels, Registry, Gauge, Encoder, TextEncoder};
use std::sync::{Arc, Mutex};
use std::io::Result;

mod thermometry;
mod app_state;
use app_state::AppState;

pub fn start(_cli_args: clap::ArgMatches) -> Result<()> {
    let sys = System::new("thermometry");

    let registry = Registry::new();
    let opts = opts!(
        "temp_celsius", "Hardware monitor for temperature (input)",
        labels!{"sensor" => "soc_chip_temp",}
    ).namespace("node").subsystem("hwmon");
    let t_gauge = Gauge::with_opts(opts).unwrap();
    registry.register(Box::new(t_gauge.clone())).unwrap();

    let app_state = AppState {
        registry: Arc::new(Mutex::new(registry)),
        t_gauge: Arc::new(Mutex::new(t_gauge))
    };
    
    let state_addr = app_state.clone().start();
    let app_data = web::Data::new(app_state);
    HttpServer::new(
        move || {
            App::new()
               .register_data(app_data.clone())
               .service(metrics)
        }
    )
    .bind("0.0.0.0:9090")?
    .start();
    
    thermometry::start(state_addr);

    sys.run()
}

#[get("/metrics")]
fn metrics(data: web::Data<AppState>) -> impl Responder {
    let registry = data.registry.lock().unwrap();

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metrics = registry.gather();
    encoder.encode(&metrics, &mut buffer).unwrap();
    
    let page = String::from_utf8(buffer).unwrap();
    HttpResponse::Ok().body(page)
}