use std::io::prelude::*;
use std::io::Error;
use actix_files::NamedFile;
use log::{trace};

use super::thermometry::Temperature;

pub fn from_file(path: String) -> Result<Temperature, Error> {
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