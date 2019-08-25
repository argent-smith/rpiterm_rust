
use actix::prelude::*;
use std::sync::{Arc, Mutex};

use super::thermometry::Temperature;
use super::prometheus::PrometheusData;

#[derive(Clone)]
pub struct AppState {
    prometheus: Arc<Mutex<PrometheusData>>,
}

impl AppState {
    pub fn new(prometheus_data: PrometheusData) -> Self {
        AppState { prometheus: Arc::new(Mutex::new(prometheus_data)) }
    }

    pub fn render_prometheus(&self) -> String {
        unpack_prometheus(&self.prometheus).render()
    } 
}

impl Actor for AppState {
    type Context = Context<Self>;
}

#[derive(Message)]
pub struct UpdateTemperatureGauge(pub Temperature);

impl Handler<UpdateTemperatureGauge> for AppState {
    type Result = ();

    fn handle(&mut self, msg: UpdateTemperatureGauge, _ctx: &mut Context<Self>) {
        unpack_prometheus(&self.prometheus).t_gauge().set(msg.0)
    }
}

fn unpack_prometheus(arc: &Arc<Mutex<PrometheusData>>) -> PrometheusData {
    arc.clone().lock().unwrap().clone()
}