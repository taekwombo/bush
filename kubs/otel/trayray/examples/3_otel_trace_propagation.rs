use std::collections::HashMap;

use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    tracing_subscriber::registry()
        .with(trayray::otel_trace_layer())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Connecting multiple spans together into single Trace.
    // For that we need propagation mechanism.
    trayray::otel_init_propagation();

    // Stores propagation information from previously executed span.
    let mut carrier: HashMap<String, String> = HashMap::new();

    span_load_and_set_context("beginning-operation", &mut carrier);
    span_load_and_set_context("sync-first-page", &mut carrier);
    span_load_and_set_context("sync-second-page", &mut carrier);

    std::thread::sleep(std::time::Duration::from_millis(5_000));
}

/// Simulates async communication where carrier is the info propagated to other components.
/// 
/// This information is used to set up OTEL Context and correct TraceID for the span.
///
/// [tracing::Span] has the trait [tracing_opentelemetry::OpenTelemetrySpanExt], therefore we can
/// attach OTEL data directly via [tracing::Span].
fn span_load_and_set_context(op_name: &str, carrier: &mut HashMap<String, String>) {
    use tracing_opentelemetry::OpenTelemetrySpanExt;

    // Extract context from carrier.
    let remote_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.extract(carrier)
    });

    // Create new span and immediately assign it's OTEL parent using OpenTelemetrySpanExt.
    let span = tracing::span!(Level::INFO, "sim-parent-child");
    span.set_parent(remote_context.clone()).unwrap();

    span.in_scope(|| {
        // Attaching event via tracing.
        tracing::info!("tracing - started {}", op_name);

        // https://docs.rs/tracing-opentelemetry/latest/tracing_opentelemetry/trait.OpenTelemetrySpanExt.html
        // Attaching otel attribute.
        span.set_attribute("otel-attr", "otel-value");
        // Attaching otel event.
        span.add_event(format!("otel - adde {}", op_name), vec![]);

        // Extract current context and inject it into our carrier while the span is active.
        opentelemetry::global::get_text_map_propagator(|propagator| {
            // propagator.inject(carrier);
        });
    });

    // OR
    // Extract span context and inject it into our carrier using span handle.
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&span.context(), carrier);
    });
}
