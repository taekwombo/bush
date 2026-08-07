//! https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/index.html#filtering-with-layers
//! https://docs.rs/tracing-subscriber/latest/tracing_subscriber/registry/struct.Registry.html

use tracing::dispatcher::Dispatch;
use tracing_subscriber::layer::{Layer, SubscriberExt};

fn main() {
    // Let's see how tracing_subscriber crate allows us to compose layers into Subscriber.

    use tracing_subscriber::filter::{EnvFilter, LevelFilter};
    use tracing::dispatcher;

    // Method `.init` installs Subscriber as global default.
    let fmt = || tracing_subscriber::fmt().with_max_level(LevelFilter::WARN).finish();

    dispatcher::with_default(&Dispatch::new(fmt()), || {
        tracing::info!("Ooops, this event is too unimportant right now.");
        tracing::warn!("At least warn event is expected.");
    });

    // Let's try to use span filter.
    let filter = || EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .parse("")
        .unwrap();

    // Layers can be composed into Subscriber using Registry.
    let sub = tracing_subscriber::registry()
        .with(filter())
        .with(tracing_subscriber::fmt::layer());

    dispatcher::with_default(&Dispatch::new(sub), || {
        tracing::info!("Info works, but debug doesn't!");
        tracing::debug!("Useful information here.");
    });

    // Let's try to change the order of layers in registry.
    let sub = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter());

    // Debug level still doesn't work.
    dispatcher::with_default(&Dispatch::new(sub), || {
        tracing::info!("Info works, but debug doesn't!");
        tracing::debug!("Useful information here.");
    });

    // Instead of global filtering, let's use per-layer filtering.
    let sub = tracing_subscriber::registry()
        // This layer will pretty print any trace with WARN or more important level.
        .with(tracing_subscriber::fmt::layer().pretty().with_filter(
            LevelFilter::WARN
        ))
        // This layer will print anyting.
        .with(tracing_subscriber::fmt::layer().compact());

    dispatcher::with_default(&Dispatch::new(sub), || {
        tracing::info!("Mildly important.");
        tracing::warn!("Highly important - printed twice.");
    });
}
