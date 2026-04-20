use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest,
    ExportTraceServiceResponse,
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

// #[poem::handler]
// async fn v1_uniq(
//     Data(format): Data<&Format>,
// ) -> Json<Vec<String>> {
//     let Format::Vortex { session, runtime } = format else {
//         return Json(vec![]);
//     };
//
//     let names = ottel_spaniel::vortex::read::read_unique_span_names(
//         session,
//         runtime,
//         0,
//         std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos() as u64,
//     ).await;
//     let mut names = names.into_iter().collect::<Vec<_>>();
//     names.sort();
//     Json(names)
// }
