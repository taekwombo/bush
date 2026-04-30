pub mod arrow;
pub mod misc;
pub mod vortex;
pub mod write;

pub use write::{Format, Sink, Stats};
pub(crate) use write::{SpanBuilder, SpanWriter};

#[derive(Debug)]
pub struct SpanData {
    pub trace_id: [u8; 16],
    pub span_id: [u8; 8],
    pub parent_span_id: Option<[u8; 8]>,
    pub name: String,
    pub kind: i32,
    pub status_code: Option<i32>,
    pub status_message: Option<String>,
    pub time_start: u64,
    pub time_end: u64,
    pub time_duration: u64,
    // TODO: revisit type
    pub resource_attributes: std::sync::Arc<Vec<opentelemetry_proto::tonic::common::v1::KeyValue>>,
}
