use std::sync::Arc;
use std::cell::LazyCell;

use arrow::datatypes::Schema;

pub const SCHEMA: LazyCell<Arc<Schema>> = LazyCell::new(create_schema);

// Simple schema:
// trace_id
// span_id
// parent_span_id
// name
// kind
// status
// scope - next
// resource - next
// attributes - next
// events - next
// links - next

fn create_schema() -> Arc<Schema> {
    use arrow::datatypes::{Field, DataType};

    let bytes = Field::new_list_field(DataType::UInt8, false);
    let trace_id = Field::new("trace_id", DataType::FixedSizeList(Arc::new(bytes), 16), false);
    let span_id = Field::new("span_id", DataType::Int64, false);
    let parent_span_id = Field::new("parent_span_id", DataType::Int64, true);
    let name = Field::new("name", DataType::Utf8View, false);
    let kind = Field::new("kind", DataType::Int32, false);
    let status_code = Field::new("status.code", DataType::Int32, true);
    let status_message = Field::new("status.message", DataType::Utf8View, true);
    let time_start = Field::new("time.start", DataType::UInt64, false);
    let time_end = Field::new("time.end", DataType::UInt64, false);
    let time_duration = Field::new("time.duration", DataType::UInt64, false);

    let columns = vec![
        trace_id,
        span_id,
        parent_span_id,
        name,
        kind,
        status_code,
        status_message,
        time_start,
        time_end,
        time_duration,
    ];

    Arc::new(Schema::new(columns))
}

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
}
