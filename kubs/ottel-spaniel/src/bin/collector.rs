use std::sync::Arc;

use poem::{handler, get, web::{Data, Json}, Route};
use opentelemetry_proto::tonic::collector::trace::v1::{ExportTraceServiceRequest, ExportTraceServiceResponse};

use ottel_spaniel::{BobbySinker, start, convert};

#[handler]
async fn v1_trace(
    Data(sinker): Data<&Arc<BobbySinker>>,
    Json(body): Json<ExportTraceServiceRequest>,
) -> Json<ExportTraceServiceResponse> {
    let spans = convert::request_to_span_data(body);

    if !spans.is_empty() {
        sinker.write(spans).await;
    }

    Json(ExportTraceServiceResponse { partial_success: None })
}

#[handler]
fn v1_trace_get() -> () {
    tracing::info!("GET /v1/traces");
}

async fn run_server(sinker: BobbySinker) {
    use poem::{Server, listener::TcpListener, EndpointExt, middleware::AddData};

    let routes = Route::new()
        .at("/v1/traces", get(v1_trace_get).post(v1_trace))
        .with(AddData::new(Arc::new(sinker)));

    tokio::select! {
        _ = Server::new(TcpListener::bind("0.0.0.0:44318")).run(routes) => {},
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Stopping server");
        },
    }
}

fn init_tracing() {
    use tracing_subscriber::prelude::*;

    tracing_subscriber::registry()
        .with(console_subscriber::spawn())
        .with(tracing_subscriber::fmt::layer().with_filter(
            tracing_subscriber::filter::LevelFilter::INFO
        ))
        .init();
}

fn main() -> () {
    init_tracing();

    const SIZE: usize = 1024;

    let run_with_arrow = async |rx| {
        let writer = ottel_spaniel::arrow::Writer::new(SIZE * 8);

        ottel_spaniel::arrow::sink(SIZE, writer, rx).await
    };

    let run_with_vortex = async |rx| {
        use vortex::VortexSessionDefault;
        use vortex::session::VortexSession;
        use vortex::io::runtime::current::CurrentThreadRuntime;
        use vortex::io::runtime::BlockingRuntime;
        use vortex::io::session::RuntimeSessionExt;

        let rt = CurrentThreadRuntime::new();
        let session = VortexSession::default().with_handle(rt.handle());
        let writer = ottel_spaniel::vortex::Writer::new(&session, &rt, SIZE * 8);

        ottel_spaniel::vortex::sink(SIZE, writer, rx).await
    };

    let use_arrow = std::env::args().rfind(|i| i == "arrow").is_some();

    if use_arrow {
        start(SIZE, run_server, run_with_arrow);
    } else {
        start(SIZE, run_server, run_with_vortex);
    }
}
