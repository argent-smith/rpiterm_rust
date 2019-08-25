use clap;
use actix::prelude::*;
use std::io::Result;
use log::info;

mod thermometry;
mod thermometers;
mod prometheus;
use self::prometheus::PrometheusData;
mod app_state;
use app_state::AppState;
mod server;

pub fn start<'a>(cli_args: clap::ArgMatches<'a>) -> Result<()> {
    let sys = System::new("thermometry");
    let prometheus_data = PrometheusData::new();
    let app_state = AppState::new(prometheus_data); 
    let state_addr = app_state.clone().start();
    let t_file = String::from(cli_args.value_of("thermometer-file").unwrap());
    thermometry::start(state_addr, t_file);
    let port: u16 = String::from(cli_args.value_of("port").unwrap())
        .parse().unwrap();
    let _ = server::start(app_state, port);
    info!("starting the actix system"); 
    sys.run()
}