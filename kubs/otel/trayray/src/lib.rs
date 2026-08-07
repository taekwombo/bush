use tracing::subscriber::Subscriber;
use tracing_subscriber::layer::Layer;
use tracing_subscriber::registry::LookupSpan;

/// [EnvResourceDetector] loads OTEL_RESOURCE_ATTRIBUTES environment variable.
fn get_resource() -> opentelemetry_sdk::Resource {
    use opentelemetry_sdk::resource::EnvResourceDetector;

    opentelemetry_sdk::Resource::builder()
        .with_service_name("ex-service")
        .with_detector(Box::new(EnvResourceDetector::new()))
        .build()
}

pub fn otel_trace_layer_batch<S>() -> impl Layer<S>
where
    S: Subscriber,
    S: for<'span> LookupSpan<'span>,
{
    use opentelemetry::trace::TracerProvider;

    let span_exporter = opentelemetry_otlp::SpanExporter::builder().build().unwrap();
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_resource(get_resource())
        .build();

    let trayray_tracer = tracer_provider.tracer("trayray");

    // Set tracer provider for the duration of the program.
    //
    // See create_span_with_custom_tracer.
    opentelemetry::global::set_tracer_provider(tracer_provider);

    tracing_opentelemetry::layer().with_tracer(trayray_tracer)
}


pub fn otel_trace_layer<S>() -> impl Layer<S>
where
    S: Subscriber,
    S: for<'span> LookupSpan<'span>,
{
    use opentelemetry::trace::TracerProvider;

    let span_exporter = opentelemetry_otlp::SpanExporter::builder().build().unwrap();
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(span_exporter)
        .with_resource(get_resource())
        .build();

    let trayray_tracer = tracer_provider.tracer("trayray");

    // Set tracer provider for the duration of the program.
    //
    // See create_span_with_custom_tracer.
    opentelemetry::global::set_tracer_provider(tracer_provider);

    tracing_opentelemetry::layer().with_tracer(trayray_tracer)
}

pub fn otel_init_propagation() {
    use opentelemetry::propagation::*;
    use opentelemetry_sdk::propagation::*;

    let trace_context_propagator = TraceContextPropagator::new();
    let baggage_propagator = BaggagePropagator::new();
    let propagator = TextMapCompositePropagator::new(vec![
        Box::new(trace_context_propagator),
        Box::new(baggage_propagator),
    ]);
    opentelemetry::global::set_text_map_propagator(propagator);
}

/// https://docs.rs/opentelemetry-appender-log/latest/opentelemetry_appender_log/index.html#getting-started
/// +
/// https://docs.rs/opentelemetry-appender-tracing/latest/opentelemetry_appender_tracing/
///
///
/// Sets both log and tracing crates support for OTEL Log export.
pub fn otel_log_appender<S>() -> (opentelemetry_sdk::logs::SdkLoggerProvider, impl Layer<S>) 
where
    S: Subscriber,
    S: for<'span> LookupSpan<'span>,
{
    let exporter = opentelemetry_otlp::LogExporter::builder().build().unwrap();
    let logger_provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_log_processor(
            opentelemetry_sdk::logs::BatchLogProcessor::builder(exporter).build(),
        )
        .build();

    let otel_log_appender = opentelemetry_appender_log::OpenTelemetryLogBridge::new(&logger_provider);

    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let tracing_layer = opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(&logger_provider);

    (logger_provider, tracing_layer)
}

pub fn otel_metrics_setup() {
    let exporter = opentelemetry_otlp::MetricExporter::builder().build().unwrap();
    let provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(get_resource())
        .build();

    opentelemetry::global::set_meter_provider(provider);
}
