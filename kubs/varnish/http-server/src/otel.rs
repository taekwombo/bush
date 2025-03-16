use opentelemetry_otlp::{ExportConfig, Protocol, WithExportConfig, WithHttpConfig};

// https://github.com/open-telemetry/opentelemetry-rust/blob/879d6ff9d251db158da7a5720ff903fa6cb7c1e2/opentelemetry-otlp/src/exporter/mod.rs#L178
fn get_otel_config() -> ExportConfig {
    use std::str::FromStr;
    use std::time::Duration;

    const DEFAULT_TIMEOUT_MS: u64 = 10_000;
    const DEFAULT_ENDPOINT_HTTP: &str = "http://localhost:4317";
    const DEFAULT_ENDPOINT_GRPC: &str = "http://localhost:4318";

    fn default_endpoint(protocol: &Protocol) -> String {
        match protocol {
            Protocol::Grpc => DEFAULT_ENDPOINT_GRPC.to_owned(),
            _ => DEFAULT_ENDPOINT_HTTP.to_owned(),
        }
    }

    // https://opentelemetry.io/docs/languages/sdk-configuration/otlp-exporter/#otel_exporter_otlp_protocol
    let protocol = match std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL") {
        Ok(ref value) => match value.as_str() {
            "http/protobuf" => Protocol::HttpBinary,
            "http/json" => Protocol::HttpJson,
            _ => Protocol::Grpc,
        },
        _ => Protocol::Grpc,
    };
    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .ok()
        .unwrap_or_else(|| default_endpoint(&protocol));
    let timeout = std::env::var("OTEL_EXPORTER_OTLP_TIMEOUT")
        .ok()
        .and_then(|s| FromStr::from_str(&s).ok())
        .unwrap_or(DEFAULT_TIMEOUT_MS);

    opentelemetry_otlp::ExportConfig {
        protocol,
        endpoint: Some(endpoint),
        timeout: Duration::from_millis(timeout),
    }
}

pub fn init() -> opentelemetry_sdk::logs::SdkLoggerProvider {
    use opentelemetry::propagation::*;
    use opentelemetry::trace::TracerProvider;
    use opentelemetry_sdk::propagation::*;
    use tracing_subscriber::prelude::*;

    let otel_cfg = get_otel_config();
    let export_config = || ExportConfig {
        protocol: otel_cfg.protocol,
        endpoint: otel_cfg.endpoint.clone(),
        timeout: otel_cfg.timeout,
    };

    let trace_context_propagator = TraceContextPropagator::new();
    let baggage_propagator = BaggagePropagator::new();
    let propagator = TextMapCompositePropagator::new(vec![
        Box::new(trace_context_propagator),
        Box::new(baggage_propagator),
    ]);
    opentelemetry::global::set_text_map_propagator(propagator);

    // Traces OTEL
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(match otel_cfg.protocol {
            Protocol::Grpc => opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_export_config(export_config())
                .build()
                .unwrap(),
            _ => opentelemetry_otlp::SpanExporter::builder()
                .with_http()
                .with_export_config(export_config())
                .with_http_client(reqwest::Client::new())
                .build()
                .unwrap(),
        })
        .build();

    let tracer = tracer_provider.tracer("kubs");
    opentelemetry::global::set_tracer_provider(tracer_provider);

    // Logs
    let log_exporter = match otel_cfg.protocol {
        Protocol::Grpc => opentelemetry_otlp::LogExporter::builder()
            .with_tonic()
            .with_export_config(export_config())
            .build()
            .unwrap(),
        _ => opentelemetry_otlp::LogExporter::builder()
            .with_http()
            .with_export_config(export_config())
            .with_http_client(reqwest::Client::new())
            .build()
            .unwrap(),
    };

    let logger_provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .build();

    // tracing-rs : traces and logs
    let otel_traces = tracing_opentelemetry::layer().with_tracer(tracer);
    let otel_logs =
        opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(&logger_provider);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::builder().from_env_lossy())
        .with(otel_traces)
        .with(otel_logs)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Metrics
    let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
        .with_periodic_exporter(match otel_cfg.protocol {
            Protocol::Grpc => opentelemetry_otlp::MetricExporter::builder()
                .with_tonic()
                .with_export_config(export_config())
                .build()
                .unwrap(),
            _ => opentelemetry_otlp::MetricExporter::builder()
                .with_http()
                .with_export_config(export_config())
                .with_http_client(reqwest::Client::new())
                .build()
                .unwrap(),
        })
        .build();

    opentelemetry::global::set_meter_provider(meter_provider);

    logger_provider
}

pub struct RocketHeadersCtx<'a> {
    headers: &'a rocket::http::HeaderMap<'a>,
    keys: Vec<String>,
}

impl<'a> RocketHeadersCtx<'a> {
    pub fn new(headers: &'a rocket::http::HeaderMap<'a>) -> Self {
        Self {
            headers,
            keys: headers
                .iter()
                .map(|h| h.name.to_string())
                .collect::<Vec<_>>(),
        }
    }
}

impl opentelemetry::propagation::Extractor for RocketHeadersCtx<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.headers.get_one(key)
    }

    fn keys(&self) -> Vec<&str> {
        self.keys.iter().map(|s| s.as_ref()).collect::<Vec<_>>()
    }
}
