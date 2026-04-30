use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
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
pub async fn v0_search_get_svc_names(
    Data(format): Data<&Format>,
    Data(stats): Data<&Stats>,
    Json(body): Json<ServiceNamesForCompletionRequest>,
) -> Json<ServiceNamesForCompletionResponse> {
    let files: Vec<Box<_>> = { stats.files.read().await.iter().cloned().collect() };
    let mut names: Sagarray<50, String> = Sagarray::new();

    match format {
        Format::Arrow => {
            use ottel_spaniel::arrow::{AsSpanData, Filter, Null, Read, columns};
            let mut read = Read::new(
                [
                    columns::RES_ATTR_NAME.name(),
                    columns::RES_ATTR_TYPE.name(),
                    columns::RES_ATTR_VALUE.name(),
                ],
                |schema| {
                    vec![
                        Box::new(Null::not_null(schema, columns::PARENT_SPAN_ID.name())),
                        Box::new(
                            Filter::new_u64(
                                schema,
                                columns::TIME_START.name(),
                                body.start_time_ms * 1_000_000,
                            )
                            .gte(),
                        ),
                        Box::new(
                            Filter::new_u64(
                                schema,
                                columns::TIME_END.name(),
                                body.end_time_ms * 1_000_000,
                            )
                            .lte(),
                        ),
                    ]
                },
                files,
            );

            'outter: while let Some(batch) = read.next_batch().await {
                for svc_name in batch.get_svc_names() {
                    if names.contains(svc_name.as_str()) {
                        continue;
                    }

                    names.push(svc_name);

                    if names.len >= names.cap {
                        break 'outter;
                    }
                }
            }
        }
        Format::Vortex { .. } => unimplemented!(),
    }

    let mut names: Vec<_> = names.into_vec();
    names.sort();
    Json(ServiceNamesForCompletionResponse { svc_names: names })
}

#[poem::handler]
pub async fn v0_search_get_span_names(
    Data(format): Data<&Format>,
    Data(stats): Data<&Stats>,
    Json(body): Json<SpanNamesForCompletionRequest>,
) -> Json<SpanNamesForCompletionResponse> {
    let files: Vec<Box<_>> = { stats.files.read().await.iter().cloned().collect() };
    let mut names: Sagarray<50, String> = Sagarray::new();

    match format {
        Format::Arrow => {
            use ottel_spaniel::arrow::{
                AsSpanData, CustomFilter, Filter, Read,
                columns::{SPAN_NAME, TIME_END, TIME_START},
            };

            let mut read = Read::new(
                [SPAN_NAME.name()],
                |schema| {
                    let mut base: Vec<Box<dyn CustomFilter>> = vec![
                        Box::new(
                            Filter::new_u64(
                                schema,
                                TIME_START.name(),
                                body.start_time_ms * 1_000_000,
                            )
                            .gte(),
                        ),
                        Box::new(
                            Filter::new_u64(schema, TIME_END.name(), body.end_time_ms * 1_000_000)
                                .lte(),
                        ),
                    ];

                    if let Some(contains) = body.contains.as_ref() {
                        base.push(Box::new(
                            Filter::new_str(schema, SPAN_NAME.name(), contains.as_str()).contains(),
                        ));
                    }

                    base
                },
                files,
            );

            'outter: while let Some(batch) = read.next_batch().await {
                for name in batch.get_names() {
                    if names.contains(name) {
                        continue;
                    }

                    names.push(name.to_owned());

                    if names.len >= names.cap {
                        break 'outter;
                    }
                }
            }
        }
        f @ Format::Vortex { .. } => {
            use ottel_spaniel::vortex::read::*;
            use vortex::expr::*;

            let mut filter = and(
                gt(
                    get_item("time_start", root()),
                    lit(body.start_time_ms * 1_000_000),
                ),
                lt(
                    get_item("time_end", root()),
                    lit(body.end_time_ms * 1_000_000),
                ),
            );

            if let Some(c) = body.contains {
                filter = and(
                    filter,
                    ilike(get_item("name", root()), lit(format!("%{c}%"))),
                );
            }

            let mut read = Read::new(f, files)
                .with_filter(filter)
                .with_projection(select(["name"], root()));

            'outter: while let Some(arr) = read.next_batch().await {
                for name in arr.get_names() {
                    let name = name.as_utf8().value().unwrap().as_str();

                    if names.contains(name) {
                        continue;
                    }

                    names.push(name.to_owned());

                    if names.len >= names.cap {
                        break 'outter;
                    }
                }
            }
        }
    }

    let mut span_names: Vec<_> = names.into_vec();
    span_names.sort();

    Json(SpanNamesForCompletionResponse { span_names })
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceNamesForCompletionRequest {
    start_time_ms: u64,
    end_time_ms: u64,
    contains: Option<String>,
    limit: u16,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceNamesForCompletionResponse {
    svc_names: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpanNamesForCompletionRequest {
    start_time_ms: u64,
    end_time_ms: u64,
    contains: Option<String>,
    limit: u16,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpanNamesForCompletionResponse {
    span_names: Vec<String>,
}

#[poem::handler]
pub async fn v0_search_traces(
    Data(_format): Data<&Format>,
    Data(_stats): Data<&Stats>,
    Json(_body): Json<SearchTracesRequest>,
) -> Json<SearchTracesResponse> {
    todo!()
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTracesRequest {
    end_time_ms: u64,
    start_time_ms: u64,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTracesResponse {
    data: u8,
}

struct Sagarray<const CAP: usize, T> {
    data: [std::mem::MaybeUninit<T>; CAP],
    len: usize,
    cap: usize,
}

impl<const CAP: usize, T> Sagarray<CAP, T> {
    fn new() -> Self {
        Self {
            data: [const { std::mem::MaybeUninit::uninit() }; CAP],
            len: 0,
            cap: CAP,
        }
    }

    fn push(&mut self, value: T) {
        assert!(self.len < CAP);
        self.data[self.len] = std::mem::MaybeUninit::new(value);
        self.len += 1;
    }

    fn into_vec(mut self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len);
        let mut ptr: std::ptr::NonNull<std::mem::MaybeUninit<T>> =
            std::ptr::NonNull::new(self.data.as_mut_slice().as_mut_ptr()).unwrap();

        for _ in 0..self.len {
            unsafe {
                let old = ptr.read();
                ptr = ptr.add(1);
                result.push(old.assume_init());
            }
        }

        result
    }
}

impl<const CAP: usize, T: PartialEq> Sagarray<CAP, T> {
    fn contains<E: ?Sized>(&self, other: &E) -> bool
    where
        T: std::ops::Deref<Target = E>,
        for<'a> &'a E: PartialEq,
    {
        for i in 0..self.len {
            unsafe {
                if self.data[i].assume_init_ref().deref() == other {
                    return true;
                }
            }
        }

        false
    }
}
