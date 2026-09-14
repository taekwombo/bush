use opentelemetry::trace::TraceContextExt;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .with(trayray::otel_trace_layer())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let span = tracing::info_span!("base_span_with_links");
    let _g = span.enter();

    tracing::info!("Entering linked span now.");

    tracing::info_span!(parent: None, "inner").in_scope(|| {
        span.add_link(tracing::Span::current().context().span().span_context().clone());
        tracing::info!("Hi from inner - we are linked with base span.");
    });

    std::thread::sleep(std::time::Duration::from_millis(5_000));
}
