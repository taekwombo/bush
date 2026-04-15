use std::sync::Arc;
use std::cell::LazyCell;

use vortex::dtype::{DType, PType, StructFields};
use vortex::dtype::Nullability::*;
use vortex::array::builders::*;
use vortex::array::arrays::*;
use vortex::scalar::Scalar;

use crate::schema::*;

pub struct FieldTypes {
    trace_id_element: Arc<DType>,
    trace_id: DType,
    span_id: DType,
    parent_span_id: DType,
    name: DType,
    kind: DType,
    status_code: DType,
    status_message: DType,
    time_start: DType,
    time_end: DType,
    time_duration: DType,
}

fn create_types() -> FieldTypes {
    let trace_id_element = Arc::new(DType::Primitive(PType::U8, NonNullable));
    let trace_id = DType::FixedSizeList(trace_id_element.clone(), 16, NonNullable);
    let span_id = DType::Primitive(PType::I64, NonNullable);
    let parent_span_id = DType::Primitive(PType::I64, Nullable);
    let name = DType::Utf8(NonNullable);
    let kind = DType::Primitive(PType::I32, NonNullable);
    let status_code = DType::Primitive(PType::I32, Nullable);
    let status_message = DType::Utf8(Nullable);

    let time_start = DType::Primitive(PType::U64, NonNullable);
    let time_end = DType::Primitive(PType::U64, NonNullable);
    let time_duration = DType::Primitive(PType::U64, NonNullable);

    FieldTypes {
        trace_id_element,
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
    }
}

pub const FIELDS: LazyCell<FieldTypes> = LazyCell::new(create_types);

fn create_struct_fields() -> StructFields {
    StructFields::from_iter([
        ("trace_id", FIELDS.trace_id.clone()),
        ("span_id", FIELDS.span_id.clone()),
        ("parent_span_id", FIELDS.parent_span_id.clone()),
        ("name", FIELDS.name.clone()),
        ("kind", FIELDS.kind.clone()),
        ("status_code", FIELDS.status_code.clone()),
        ("status_message", FIELDS.status_message.clone()),
        ("time_start", FIELDS.time_start.clone()),
        ("time_end", FIELDS.time_end.clone()),
        ("time_duration", FIELDS.time_duration.clone()),
    ])
}

pub const STRUCT_FIELDS: LazyCell<StructFields> = LazyCell::new(create_struct_fields);
pub const STRUCT: LazyCell<DType> = LazyCell::new(|| DType::Struct(STRUCT_FIELDS.clone(), NonNullable));

fn to_scalar(data: SpanData) -> Scalar {
    let trace_id: Vec<Scalar> = data.trace_id
        .iter()
        .map(|v| Scalar::primitive(*v, NonNullable))
        .collect();

    Scalar::struct_(
        STRUCT.clone(),
        vec![
            Scalar::fixed_size_list(
                FIELDS.trace_id_element.clone(),
                trace_id,
                NonNullable,
            ),
            Scalar::primitive(i64::from_be_bytes(data.span_id), NonNullable),
            data.parent_span_id
                .map(i64::from_be_bytes)
                .map(|v| Scalar::primitive(v, Nullable))
                .unwrap_or_else(Scalar::null_native::<i64>),
            Scalar::utf8(data.name, NonNullable),
            Scalar::primitive(data.kind, NonNullable),
            data.status_code
                .map(|v| Scalar::primitive(v, Nullable))
                .unwrap_or_else(Scalar::null_native::<i64>),
            data.status_message
                .map(|v| Scalar::utf8(v.as_str(), Nullable))
                .unwrap_or(Scalar::null(FIELDS.status_message.clone())),
            Scalar::primitive(data.time_start, NonNullable),
            Scalar::primitive(data.time_end, NonNullable),
            Scalar::primitive(data.time_duration, NonNullable),
        ],
    )
}

pub struct Builder {
    builder: StructBuilder,
    pub size: usize,
    pub threshold: usize,
}

impl Builder {
    pub fn new(threshold: usize, capacity: usize) -> Self {
        Self {
            builder: StructBuilder::with_capacity(STRUCT_FIELDS.clone(), NonNullable, capacity),
            size: 0,
            threshold,
        }
    }

    pub fn append(&mut self, data: Vec<SpanData>) -> bool {
        self.size += data.len();

        for data in data {
            self.builder.append_value(to_scalar(data).as_struct()).expect("append.struct.ok");
        }

        self.size >= self.threshold
    }

    pub fn build(&mut self) -> StructArray {
        self.size = 0;
        self.builder.finish_into_struct()
    }
}
