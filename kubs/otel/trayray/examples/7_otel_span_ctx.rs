/// [opentelemetry::Context] can be used to provide information between spans indirectly.
///
/// Information set by span can be accessed by descendant spans.

use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug)]
struct CtxInfo {
    #[allow(unused)]
    name: &'static str,
}

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .with(trayray::otel_trace_layer())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let span = tracing::info_span!("outter");
    let _g = span.enter();

    let context = span.context().with_value(CtxInfo { name: "outter" });
    let _cg = context.attach();

    tracing::info!("Entering child span now.");

    tracing::info_span!("inner").in_scope(|| {
        let ctx = tracing::Span::current().context();
        let info = ctx.get::<CtxInfo>();
        tracing::info!("Inner context is: {:#?}", ctx);
        tracing::info!("Inner info is: {:#?}", info);
    });

    std::thread::sleep(std::time::Duration::from_millis(5_000));
}
