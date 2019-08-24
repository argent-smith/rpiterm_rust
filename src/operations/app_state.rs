
use actix::prelude::*;
use std::sync::{Arc, Mutex};
use prometheus::{Registry, Gauge};

use super::thermometry::Temperature;

#[derive(Clone)]
pub struct AppState {
    pub registry: Arc<Mutex<Registry>>,
    pub t_gauge: Arc<Mutex<Gauge>>,
}

impl Actor for AppState {
    type Context = Context<Self>;
}

#[derive(Message)]
pub struct UpdateTemperatureGauge(pub Temperature);

impl Handler<UpdateTemperatureGauge> for AppState {
    type Result = ();

    fn handle(&mut self, msg: UpdateTemperatureGauge, _ctx: &mut Context<Self>) {
        let gauge = self.t_gauge.clone().lock().unwrap().clone();
        gauge.set(msg.0);
    }
}