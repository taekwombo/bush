use opentelemetry_sdk::Resource
use opentelemetry_otlp::SpanExporter;

fn init(service_name: &str) {
    let resource = Resource::builder().with_service_name(service_name).build();

    // Traces
    let span_exporter = opentelemetry_otlp::SpanExporter::builder().build().unwrap();
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .build();
}
