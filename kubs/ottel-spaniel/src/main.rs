use poem::{handler, get, web::{Data, Json}, Route};
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use tokio::sync::oneshot;

use ottel_spaniel::*;

#[handler]
async fn v1_trace(Data(sender): Data<&std::sync::Arc<Sender>>, Json(body): Json<ExportTraceServiceRequest>) -> () {
    let spans = convert::request_to_span_data(body);

    if spans.is_empty() {
        return;
    }

    let (tx, rx) = oneshot::channel();
    sender.send((tx, spans)).await.unwrap();
    rx.await.expect("ok");
}

#[handler]
fn v1_trace_get() -> () {
    println!("GET /v1/traces");
}

#[tokio::main]
async fn main() {
    use poem::{Server, listener::TcpListener, EndpointExt, middleware::AddData};

    let (sender, task) = create_writer(100);

    let routes = Route::new()
        .at("/v1/traces", get(v1_trace_get).post(v1_trace))
        .with(AddData::new(std::sync::Arc::new(sender)));

    let server = Server::new(TcpListener::bind("0.0.0.0:44318")).run(routes);

    tokio::select! {
        e = server => {
            println!("S result: {:?}", e);
        }
        e = task => {
            println!("S result: {:?}", e);
        }
    };
}
