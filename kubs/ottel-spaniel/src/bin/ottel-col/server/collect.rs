use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
};
use poem::web::{Data, Json};

use ottel_spaniel::Sink;

#[poem::handler]
pub async fn v1_handle_export_trace_request(
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
