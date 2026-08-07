use tracing::subscriber::Subscriber;
use tracing_subscriber::layer::Layer;
use tracing_subscriber::registry::LookupSpan;

fn main() {
}

fn otel_trace_layer<S>() -> impl Layer<S>
where
    S: Subscriber,
    S: for<'span> LookupSpan<'span>,
{
    tracing_opentelemetry::layer()
}
