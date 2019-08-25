use clap;
use actix::prelude::*;
use actix_web::{
    get, 
    web,
    App, 
    HttpServer, 
    Responder, 
    HttpResponse,
    middleware::Logger,
};
use std::io::Result;
use log::info;

mod thermometry;
mod thermometers;
mod prometheus;
use self::prometheus::PrometheusData;
mod app_state;
use app_state::AppState;

pub fn start<'a>(cli_args: clap::ArgMatches<'a>) -> Result<()> {
    let sys = System::new("thermometry");
    
    let prometheus_data = PrometheusData::new();
    let app_state = AppState::new(prometheus_data);
    
    info!("starting the app state actor"); 
    let state_addr = app_state.clone().start();

    info!("starting the http server"); 
    let app_data = web::Data::new(app_state);
    HttpServer::new(
        move || {
            App::new()
                .wrap(Logger::default())
                .register_data(app_data.clone())
                .service(metrics)
        }
    )
    .bind("0.0.0.0:9090")?
    .start();
    
    let t_file = String::from(cli_args.value_of("thermometer-file").unwrap());
    thermometry::start(state_addr, t_file);

    info!("starting the actix system"); 
    sys.run()
}

#[get("/metrics")]
fn metrics(data: web::Data<AppState>) -> impl Responder { 
    let page = data.clone().render_prometheus();
    HttpResponse::Ok().body(page)
}
