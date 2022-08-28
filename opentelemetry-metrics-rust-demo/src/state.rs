use opentelemetry::Context;

use opentelemetry::metrics::{Counter, ValueRecorder};

#[derive(Clone)]
pub struct AppState {
    pub metric: Metric,
}

#[derive(Clone)]
pub struct Metric {
    pub cx: Context,
    pub http_counter: Counter<u64>,

    // migrate from ValueRecorder to Histogram if opentelemetry 0.18.0 released
    pub http_req_histogram: ValueRecorder<f64>,
}
