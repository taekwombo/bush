//! In this example we will bridge tracing and log data to OTEL Logs.

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    let (_logger_provider, otel_tracing_log_layer) = trayray::otel_log_appender();

    tracing_subscriber::registry()
        .with(trayray::otel_trace_layer())
        .with(otel_tracing_log_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Normally tracing_subscriber will bridge log crate events into tracing events.
    // That is, if tracing-log feature flag is provided.
    //
    // Default bridging log -> tracing -> OTEL seems good enough.
    tracing::info_span!("bridging-log-tracing").in_scope(|| {
        log::info!("Hello there from log crate - exporting to OTEL Logs");
        tracing::info!("Hello there from tracing crate");
    });

    std::thread::sleep(std::time::Duration::from_millis(5_000));
}
