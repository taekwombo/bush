use tracing::Level;
use tracing_subscriber::layer::SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    tracing_subscriber::registry()
        .with(trayray::otel_trace_layer())
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::span!(Level::INFO, "operation").in_scope(|| {
        tracing::info!("TEST");

        create_span_with_custom_tracer("parser");
    });

    std::thread::sleep(std::time::Duration::from_millis(5_000));
}

/// Simluates another crate using OTEL traces.
/// 
/// There needs to be [opentelemetry::global::set_tracer_provider] called in order for us to get
/// valid tracer provider instead of Noop implementation.
///
/// See examples in: https://docs.rs/opentelemetry/latest/opentelemetry/global/index.html
fn create_span_with_custom_tracer(tracer_name: &str) {
    use opentelemetry::global;
    use opentelemetry::trace::Tracer;
    use opentelemetry::InstrumentationScope;

    let scope = InstrumentationScope::builder(tracer_name.to_string())
        .with_version("0.0.1")
        .build();

    let tracer = global::tracer_with_scope(scope);

    // Note that this operates on OTEL Span which is different from tracing Span.
    //
    // OTEL span have dynamic attributes, while tracing Span must have known attributes in front.
    tracer.in_span("another-library-span", |ctx| {
        use opentelemetry::trace::TraceContextExt;

        ctx.span().set_attribute(opentelemetry::KeyValue::new("my-attr", "my-val"));

        tracing::info!("ehh");
    });
}
