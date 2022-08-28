# opentelemetry-metrics-rust-demo

this is an opentelemetry metrics demo for [axum](https://github.com/tokio-rs/axum/) framework.

the demo add a custom opentelemetry metrics middleware `metric` to axum framework, and setup a prometheus exporter endpoint at `/metrics`.

## warning

according https://opentelemetry.io/docs/instrumentation/rust/

the status Metrics component for OpenTelemetry Rust is **Alpha** currently, use it at your own risk

## docs

https://uptrace.dev/opentelemetry/prometheus-metrics.html#sending-go-metrics-to-prometheus

https://opentelemetry.io/docs/reference/specification/metrics/sdk/

https://github.com/tokio-rs/axum/tree/main/examples/prometheus-metrics

https://github.com/labstack/echo-contrib/blob/master/prometheus/prometheus.go

semconv

https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/resource/semantic_conventions/README.md#document-conventions
