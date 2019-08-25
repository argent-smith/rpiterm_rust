use clap;
use actix::prelude::*;
use actix_web::{
    get, 
    web,
    App, 
    HttpServer, 
    Responder, 
    HttpResponse
};
use std::io::Result;

mod thermometry;
mod thermometers;
mod prometheus;
use self::prometheus::PrometheusData;
mod app_state;
use app_state::AppState;

pub fn start(_cli_args: clap::ArgMatches) -> Result<()> {
    let sys = System::new("thermometry");
    
    let prometheus_data = PrometheusData::new();
    let app_state = AppState::new(prometheus_data);
    
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
    let page = data.clone().render_prometheus();
    HttpResponse::Ok().body(page)
}
