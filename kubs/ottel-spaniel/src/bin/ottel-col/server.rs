use std::time::Duration;

use poem::{EndpointExt, Server, Route};
use poem::listener::TcpListener;
use poem::middleware::*;

// for handler
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest,
    ExportTraceServiceResponse,
};
use poem::{get, post};
use poem::web::{Data, Json};
// for handler

use ottel_spaniel::write::{Format, Sink};

pub async fn run_server(format: Format, sink: Sink) {
    let routes = Route::new()
        .at("/v1/traces", post(v1_trace))
        .at("/v1/name/unique", get(v1_uniq))
        .with(Cors::default())
        .with(AddData::new(sink))
        .with(AddData::new(format));

    let tcp = TcpListener::bind("0.0.0.0:44318");
    let server = Server::new(tcp);
    let signal = async {
        tokio::signal::ctrl_c().await.unwrap();
    };

    server
        .run_with_graceful_shutdown(routes, signal, Some(Duration::from_secs(60)))
        .await
        .expect("server.closes");
}

#[poem::handler]
async fn v1_trace(
    Data(sink): Data<&Sink>,
    Json(body): Json<ExportTraceServiceRequest>,
) -> Json<ExportTraceServiceResponse> {
    let spans = crate::convert::request_to_span_data(body);

    if !spans.is_empty() {
        sink.send(spans).await;
    }

    Json(ExportTraceServiceResponse {
        partial_success: None,
    })
}


#[poem::handler]
async fn v1_uniq(
    Data(format): Data<&Format>,
) -> Json<Vec<String>> {
    let Format::Vortex { session, runtime } = format else {
        return Json(vec![]);
    };

    let names = ottel_spaniel::vortex::read::read_unique_span_names(
        session,
        runtime,
        0,
        std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos() as u64,
    ).await;
    let mut names = names.into_iter().collect::<Vec<_>>();
    names.sort();
    Json(names)
}
