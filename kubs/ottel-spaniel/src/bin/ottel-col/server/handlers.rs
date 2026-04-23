use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest,
    ExportTraceServiceResponse,
};
use poem::web::{Data, Json};

use ottel_spaniel::{Format, Sink, Stats};

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

#[poem::handler]
pub async fn v0_search_get_span_names(
    Data(format): Data<&Format>,
    Data(stats): Data<&Stats>,
    Json(body): Json<SpanNamesForCompletionRequest>,
) -> Json<SpanNamesForCompletionResponse> {
    use std::collections::HashSet;

    let files: Vec<Box<_>> = {
        stats.files.read().await.iter().map(|p| p.clone()).collect()
    };
    let mut names = HashSet::<String>::with_capacity(body.limit.into());

    match format {
        Format::Arrow => {
            use ottel_spaniel::schema::AsSpanData;
            use ottel_spaniel::arrow::{Filter, Read};

            let mut read = Read::new(
                ["name"],
                |schema| vec![
                    Box::new(Filter::new_u64(schema, "time_start", body.start_time_ms * 1_000_000).gte()),
                    Box::new(Filter::new_u64(schema, "time_end", body.end_time_ms * 1_000_000).lte()),
                    Box::new(Filter::new_str(schema, "name", body.contains.as_str()).contains()),
                ],
                files,
            );

            'outter: while let Some(batch) = read.next_batch().await {
                for name in batch.get_names() {
                    names.insert(name.to_owned());

                    if names.len() >= body.limit as usize {
                        break 'outter;
                    }
                }
            }
        },
        Format::Vortex { .. } => unimplemented!(),
    }

    let mut span_names: Vec<_> = names.into_iter().collect();
    span_names.sort();

    Json(SpanNamesForCompletionResponse { span_names })
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpanNamesForCompletionRequest {
    start_time_ms: u64,
    end_time_ms: u64,
    contains: String,
    limit: u16,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpanNamesForCompletionResponse {
    span_names: Vec<String>,
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
