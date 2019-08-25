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

use super::app_state::AppState;

pub fn start(app_state: AppState) -> Result<()> {
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
    Ok(())
}

#[get("/metrics")]
fn metrics(data: web::Data<AppState>) -> impl Responder { 
    let page = data.clone().render_prometheus();
    HttpResponse::Ok().body(page)
}
