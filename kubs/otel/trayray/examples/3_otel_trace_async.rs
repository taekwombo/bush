use tracing::Instrument;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

async fn sleep(name: &str) {
    let span = tracing::info_span!("sleepy", name);

    tokio::time::sleep(std::time::Duration::from_millis(1_000))
        .instrument(span)
        .await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(trayray::otel_trace_layer_batch())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Async operations need to be instrumented because another operations will work when await
    // point is crossed.
    // 
    // See:
    // https://docs.rs/tracing/latest/tracing/struct.Span.html#in-asynchronous-code
    let async_job_1 = async  {
        let span = tracing::info_span!("manual-span");
        let active = span.enter();

        // We won't count sleep as part of this job.
        // But at the same time, sleepy span will not be marked as a child of manual-span.
        drop(active);
        sleep("uninstrumented").await;

        // To do that, we need to use [tracing::Instrument].
        sleep("instrumented").instrument(span).await;
    };

    async_job_1.await;

    tokio::time::sleep(std::time::Duration::from_millis(10_000)).await;

    // This example fails if we're using SimpleSpanExporter using HTTP (reqwest crate).
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

