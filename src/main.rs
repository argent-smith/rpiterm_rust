use clap::{crate_authors, crate_name, crate_version, Arg};
use std::env;
use log::error;
use std::io::Result;

mod operations;

fn main() -> Result<()> {
    let cli_args = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(
            Arg::with_name("port")
                .short("p")
                .help("exporter port")
                .default_value("9090")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("thermometer-file")
                .short("f")
                .help("file containing the system thermometer data")
                .default_value("/sys/class/thermal/thermal_zone0/temp")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("verbose logging")
                .takes_value(false)
        )
        .get_matches();

    if cli_args.is_present("verbose") {
        env::set_var(
            "RUST_LOG",
            format!("{}=trace,actix_web=info", crate_name!())    
        );
    } else {
        env::set_var(
            "RUST_LOG",
            format!("{}=info,actix_web=info", crate_name!())    
        );
    }

    env_logger::init();
    operations::start(cli_args).or_else(|e| {
        error!("operations errored: {}", e);
        Err(e)
    })
}