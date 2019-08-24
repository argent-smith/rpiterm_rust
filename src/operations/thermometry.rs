use std::time::Duration;
use actix::prelude::*;

use super::app_state::{AppState, UpdateTemperatureGauge};

pub type Temperature = f64;

pub struct Thermometry {
    app_state_addr: Addr<AppState>,
}

impl Thermometry {
    fn update_curr_temperature(&mut self, _ctx: &mut Context<Self>) {
        let pipeline = self.app_state_addr
            .send(UpdateTemperatureGauge(36.6))
            .map_err(|_| ());
        Arbiter::spawn(pipeline);
    }
}

impl Actor for Thermometry {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        IntervalFunc::new(Duration::from_secs(10), Self::update_curr_temperature)
            .finish()
            .spawn(ctx);
    }
}

pub fn start(addr: Addr<AppState>) {
    Thermometry::start(Thermometry { app_state_addr: addr });
}