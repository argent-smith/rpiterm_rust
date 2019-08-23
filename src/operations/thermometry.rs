use std::io::Error;
use actix::prelude::*;

type Temperature = f64;

pub struct Thermometry;

impl Actor for Thermometry {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result="Result<Temperature, Error>")]
pub struct GetTemperature;

impl Handler<GetTemperature> for Thermometry {
    type Result = Result<Temperature, Error>;

    fn handle(&mut self, _msg: GetTemperature, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(36.6)
    }
}