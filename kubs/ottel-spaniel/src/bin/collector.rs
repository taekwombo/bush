use std::sync::Arc;

use poem::{handler, get, web::{Data, Json}, Route};
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;

use ottel_spaniel::*;

#[handler]
async fn v1_trace(Data(sinker): Data<&Arc<BobbySinker>>, Json(body): Json<ExportTraceServiceRequest>) -> () {
    let spans = convert::request_to_span_data(body);

    if spans.is_empty() {
        return;
    }

    sinker.write(spans).await;
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
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn main() -> () {
    init_tracing();

    const SIZE: usize = 1024;

    start(SIZE, run_server, async |rx| {
        let writer = arrow::Writer::new();

        arrow::sink(SIZE, writer, rx).await
    });
}
