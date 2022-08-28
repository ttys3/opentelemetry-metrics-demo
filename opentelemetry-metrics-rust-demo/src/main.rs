mod middleware;
mod state;

use std::{env, net::SocketAddr, time::Duration};

use axum::{routing::get, Router};

use opentelemetry::sdk::resource;
use opentelemetry::{global, Context, KeyValue};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rand::Rng;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // init global meter provider and prometheus exporter
    let other = resource::Resource::new(vec![KeyValue::new(
        "service.name",
        env!("CARGO_CRATE_NAME"),
    )]);
    let res = resource::Resource::default().merge(&other);
    let exporter = opentelemetry_prometheus::exporter()
        .with_default_histogram_boundaries(vec![
            0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ])
        .with_resource(res)
        .init();

    let meter = global::meter("my-app");
    let http_counter = meter
        .u64_counter(format!("{}.requests_total", env!("CARGO_CRATE_NAME")))
        .with_description("How many HTTP requests processed, partitioned by status code and HTTP method.")
        .init();

    // migrate from f64_value_recorder to f64_histogram if opentelemetry 0.18.0 released
    let histogram = meter
        .f64_value_recorder(format!("{}.request_duration_seconds", env!("CARGO_CRATE_NAME")))
        .with_description("The HTTP request latencies in seconds.")
        .init();

    let state = state::AppState {
        metric: state::Metric {
            cx: Context::new(),
            http_counter,
            http_req_histogram: histogram,
        },
    };

    let app = Router::new()
        .route("/", get(|| async { "hello world" }))
        .route("/fast", get(|| async { "fast" }))
        .route(
            "/slow",
            get(|| async {
                let num = rand::thread_rng().gen_range(0..10);
                tokio::time::sleep(Duration::from_secs(1 * num)).await;
                format!("slow: {}s", num)
            }),
        )
        .merge(middleware::metrics::routes(exporter))
        .route_layer(axum::middleware::from_fn(move |req, next| {
            middleware::metrics::track_metrics(req, next, state.clone())
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
