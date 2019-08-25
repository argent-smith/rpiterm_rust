use std::io::prelude::*;
use std::io::Error;
use rand::prelude::*;
use actix_files::NamedFile;
use log::{trace};

use super::thermometry::Temperature;

pub fn from_file(path: &str) -> Result<Temperature, Error> {
    let mut file = NamedFile::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let reading = contents.trim_end()
        .parse()
        .unwrap_or(0.0);    
    let temperature = reading / 1000.0;
    trace!("read temperature as {}", temperature);
    Ok(temperature)
}

pub fn random() -> Result<Temperature, Error> {
    let mut rng = rand::thread_rng();
    let n: f64 = rng.gen();
    let temperature = n * 100.0;
    trace!("read temperature as {}", temperature);
    Ok(temperature)
}