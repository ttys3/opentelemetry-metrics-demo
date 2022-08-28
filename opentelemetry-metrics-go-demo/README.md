# opentelemetry-metrics-go-demo

this is an opentelemetry metrics demo for echo http framework.

the demo add a custom opentelemetry metrics middleware `otelmetric` to echo framework, and setup a prometheus exporter endpoint at `/metrics`.

this `otelmetric` middleware can work as a replacement for `github.com/labstack/echo-contrib/prometheus`

## docs

https://github.com/labstack/echo-contrib/blob/master/prometheus/prometheus.go

https://uptrace.dev/opentelemetry/go-metrics.html

https://uptrace.dev/opentelemetry/prometheus-metrics.html#sending-go-metrics-to-prometheus

https://opentelemetry.io/docs/reference/specification/metrics/sdk/

https://echo.labstack.com/cookbook/hello-world/

semconv

https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/resource/semantic_conventions/README.md#document-conventions
https://github.com/open-telemetry/opentelemetry-go/blob/main/semconv/v1.12.0/resource.go
