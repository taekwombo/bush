use poem::web::{Data, Json};

use ottel_spaniel::{Format, Stats};

#[poem::handler]
pub async fn v0_search_get_svc_names(
    Data(format): Data<&Format>,
    Data(stats): Data<&Stats>,
    Json(body): Json<request::NameFilter>,
) -> Json<response::Names> {
    let files: Vec<Box<_>> = { stats.files.read().await.iter().cloned().collect() };
    let mut names: Sagarray<50, String> = Sagarray::new();

    match format {
        Format::Arrow => {
            use ottel_spaniel::arrow::{AsSpanData, Filter, Null, Read, columns};
            let mut read = Read::new(
                Some([
                    columns::RES_ATTR_NAME.name(),
                    columns::RES_ATTR_TYPE.name(),
                    columns::RES_ATTR_VALUE.name(),
                ]),
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
    Json(response::Names { names })
}

#[poem::handler]
pub async fn v0_search_get_span_names(
    Data(format): Data<&Format>,
    Data(stats): Data<&Stats>,
    Json(body): Json<request::NameFilter>,
) -> Json<response::Names> {
    let files: Vec<Box<_>> = { stats.files.read().await.iter().cloned().collect() };
    let mut names: Sagarray<50, String> = Sagarray::new();

    match format {
        Format::Arrow => {
            use ottel_spaniel::arrow::{
                AsSpanData, CustomFilter, Filter, Read,
                columns::{SPAN_NAME, TIME_END, TIME_START},
            };

            let mut read = Read::new(
                Some([SPAN_NAME.name()]),
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

    Json(response::Names { names: span_names })
}

#[poem::handler]
pub async fn v0_search_traces(
    Data(format): Data<&Format>,
    Data(stats): Data<&Stats>,
    Json(body): Json<request::TraceFilter>,
) -> Json<response::Traces> {
    // TODO: Should return top level spans only.

    let files: Vec<Box<_>> = { stats.files.read().await.iter().cloned().collect() };
    let mut traces: Sagarray<50, Span> = Sagarray::new();

    if let Format::Vortex { .. } = format {
        return Json(response::Traces { traces: vec![] });
    }

    use ottel_spaniel::arrow::{
        AsSpanData, Filter, Read,
        columns::{TIME_END, TIME_START},
        ext::*,
    };

    let mut read = Read::new(
        None::<Vec<&str>>,
        |schema| {
            vec![
                Box::new(
                    Filter::new_u64(schema, TIME_START.name(), body.start_time_ms * 1_000_000)
                        .gte(),
                ),
                Box::new(
                    Filter::new_u64(schema, TIME_END.name(), body.end_time_ms * 1_000_000).lte(),
                ),
            ]
        },
        files,
    );

    'outter: while let Some(batch) = read.next_batch().await {
        for span in batch.get_spans() {
            traces.push(span);

            if traces.len >= traces.cap {
                break 'outter;
            }
        }
    }

    Json(response::Traces {
        traces: traces.into_vec(),
    })
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
                let old: std::mem::MaybeUninit<T> = ptr.read();
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

pub mod request {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NameFilter {
        pub start_time_ms: u64,
        pub end_time_ms: u64,
        pub contains: Option<String>,
        pub limit: u8,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TraceFilter {
        pub start_time_ms: u64,
        pub end_time_ms: u64,
        pub limit: u8,
    }
}

pub mod response {
    #[derive(Debug, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Names {
        pub names: Vec<String>,
    }

    #[derive(Debug, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Traces {
        pub traces: Vec<ottel_spaniel::arrow::ext::Span>,
    }
}
