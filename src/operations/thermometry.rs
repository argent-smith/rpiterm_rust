use std::time::Duration;
use actix::prelude::*;
use log::{info, trace, error};

use super::thermometers;
use super::app_state::{AppState, UpdateTemperatureGauge};

pub type Temperature = f64;

pub struct Thermometry {
    app_state_addr: Addr<AppState>,
    t_file: String,
}

impl Thermometry {
    fn update_curr_temperature(&mut self, _ctx: &mut Context<Self>) {
        let _ = thermometers::from_file(self.t_file.clone())
            .map(|temperature| {
                trace!("sending temperature {} to app state", temperature);
                let pipeline = self.app_state_addr
                    .send(UpdateTemperatureGauge(temperature))
                    .map_err(|_| ());
                Arbiter::spawn(pipeline);
            })
            .map_err(|e| {
                error!("thermometer errored: {}", e)
            });
    }
}

impl Actor for Thermometry {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        info!("staring thermometry routine");
        self.update_curr_temperature(ctx);
        IntervalFunc::new(Duration::from_secs(10), Self::update_curr_temperature)
            .finish()
            .spawn(ctx);
    }
}

pub fn start(addr: Addr<AppState>, t_file: String) {
    Thermometry::start(Thermometry { app_state_addr: addr, t_file: t_file });
}
