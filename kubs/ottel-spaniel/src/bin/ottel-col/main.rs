use ottel_spaniel::write::{Format, Options, start_writer};

mod convert;
mod runtime;
mod server;

fn main() {
    init_tracing();

    let format = get_format();
    let options = Options {
        flush_interval_millis: 2_000,
        suspend_interval_millis: 180_000,
        suspend_after: 10,
        sink_channel_size: 256,
        request_waitlist_size: 128,
        spans_per_file: 1024 * 10,
        builder_flush_threshold: 1024,
        builder_capacity: 2048,
    };

    let server_options = server::Options {
        host: "0.0.0.0",
        port: 44318,
        shutdown_timeout_secs: 60,
    };

    let (sink, stats, task) = start_writer(&format, options);

    let server_task = server::run_server(server_options, format.clone(), stats.clone(), sink);

    let rt = runtime::RT::new();
    rt.run_server_future(server_task);
    rt.run_writer_future(task);
}

fn get_format() -> Format {
    let use_arrow = std::env::args().rfind(|i| i == "arrow").is_some();

    if use_arrow {
        return Format::Arrow;
    }

    use vortex::VortexSessionDefault;
    use vortex::io::runtime::BlockingRuntime;
    use vortex::io::runtime::current::CurrentThreadRuntime;
    use vortex::io::session::RuntimeSessionExt;
    use vortex::session::VortexSession;

    let runtime = CurrentThreadRuntime::new();
    let session = VortexSession::default().with_handle(runtime.handle());

    Format::Vortex { runtime, session }
}

fn init_tracing() {
    use tracing_subscriber::prelude::*;

    tracing_subscriber::registry()
        .with(console_subscriber::spawn())
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
        .init();
}
