use log::{trace, error};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read(path: &str) -> Result<f64, std::io::Error> {
    trace!("reading thermometer at {}", path);

    let file = File::open(path).or_else(|err| {
        error!("failed open at {}: {:?}", path, err);
        Err(err)
    })?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let reading = contents.trim_end()
        .parse()
        .unwrap_or(0.0);
    
    Ok(reading)
}
