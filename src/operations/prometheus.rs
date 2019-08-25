use prometheus::{opts, labels, Registry, Gauge, Encoder, TextEncoder};

#[derive(Clone)]
pub struct PrometheusData {
    registry: Registry,
    t_gauge: Gauge,
}

impl PrometheusData {
    pub fn t_gauge(self) -> Gauge {
        self.t_gauge
    }

    pub fn new() -> Self {
        let registry = Registry::new();
        let opts = opts!(
            "temp_celsius", "Hardware monitor for temperature (input)",
            labels!{"sensor" => "soc_chip_temp",}
        ).namespace("node").subsystem("hwmon");
        let t_gauge = Gauge::with_opts(opts).unwrap();
        registry.register(Box::new(t_gauge.clone())).unwrap();

        PrometheusData {
            registry: registry,
            t_gauge: t_gauge,
        }
    }

    pub fn render(self) -> String {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metrics = self.registry.gather();
        encoder.encode(&metrics, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}