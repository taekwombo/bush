use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(trayray::otel_trace_layer())
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::span!(Level::INFO, "operation").in_scope(|| {
        tracing::info!("We doin async!");
    });

    // This fails if we're using SimpleSpanExporter using HTTP (reqwest crate).
    // https://docs.rs/reqwest/latest/src/reqwest/blocking/wait.rs.html
    //
    // ```rs
    // fn enter() {
    //     // Check we aren't already in a runtime
    //     #[cfg(debug_assertions)]
    //     {
    //         let _enter = tokio::runtime::Builder::new_current_thread()
    //             .build()
    //             .expect("build shell runtime")
    //             .enter();
    //     }
    // }
    // ```
}

