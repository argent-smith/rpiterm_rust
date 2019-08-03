use clap;
use futures::future::{done, ok, Future};
use log::info;
use prometheus_exporter_base::{render_prometheus, MetricType, PrometheusMetric};

mod thermometer;

pub fn start(cli_args: clap::ArgMatches) {
    let bind = cli_args.value_of("port").unwrap();    
    let bind = u16::from_str_radix(&bind, 10).expect("port must be a valid number");
    let addr = ([0, 0, 0, 0], bind).into();
    
    let thermometer_path = cli_args.value_of("thermometer-file").unwrap().to_owned();

    info!("starting exporter on {}", addr);
    render_prometheus(&addr, thermometer_path, move |request, thermometer_path| {
        Box::new({
            info!("serving {:?}", request);
            done(thermometer::read(thermometer_path.as_ref()))
                .from_err()
                .and_then(|temperature| {
                    ok(set_gauge(temperature))
                })
        })
    });
}

fn set_gauge(temperature: f64) -> String {
    let gauge_name = "node_hwmon_temp_celsius";
    let sensor_label = ("sensor", "soc_chip_temp");
    let metric = PrometheusMetric::new(
        gauge_name,
        MetricType::Gauge,
        "Hardware monitor for temperature (input)"
    );
    let mut report = metric.render_header();

    let mut attributes = Vec::new();
    attributes.push(sensor_label);
    report.push_str(&metric.render_sample(Some(&attributes), temperature));
    report
}