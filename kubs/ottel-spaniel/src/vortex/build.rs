use std::sync::Arc;

use vortex::array::arrays::*;
use vortex::array::builders::*;
use vortex::dtype::Nullability::*;
use vortex::dtype::{DType, PType, StructFields};
use vortex::scalar::Scalar;

use crate::SpanBuilder;
use crate::schema::SpanData;

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

fn create_struct_fields() -> StructFields {
    let fields = create_types();

    StructFields::from_iter([
        ("trace_id", fields.trace_id.clone()),
        ("span_id", fields.span_id.clone()),
        ("parent_span_id", fields.parent_span_id.clone()),
        ("name", fields.name.clone()),
        ("kind", fields.kind.clone()),
        ("status_code", fields.status_code.clone()),
        ("status_message", fields.status_message.clone()),
        ("time_start", fields.time_start.clone()),
        ("time_end", fields.time_end.clone()),
        ("time_duration", fields.time_duration.clone()),
    ])
}

pub fn create_struct_dtype() -> DType {
    DType::Struct(create_struct_fields(), NonNullable)
}

pub struct Builder {
    builder: StructBuilder,
    dtype: DType,
    field_types: FieldTypes,
    pub size: usize,
    pub threshold: usize,
}

impl Builder {
    pub fn new(threshold: usize, capacity: usize) -> Self {
        let field_types = create_types();
        let struct_fields = create_struct_fields();
        let dtype = create_struct_dtype();

        Self {
            builder: StructBuilder::with_capacity(struct_fields, NonNullable, capacity),
            size: 0,
            threshold,
            dtype,
            field_types,
        }
    }

    fn to_scalar(&self, data: SpanData) -> Scalar {
        let trace_id: Vec<Scalar> = data
            .trace_id
            .iter()
            .map(|v| Scalar::primitive(*v, NonNullable))
            .collect();

        Scalar::struct_(
            self.dtype.clone(),
            vec![
                Scalar::fixed_size_list(
                    self.field_types.trace_id_element.clone(),
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
                    .unwrap_or(Scalar::null(self.field_types.status_message.clone())),
                Scalar::primitive(data.time_start, NonNullable),
                Scalar::primitive(data.time_end, NonNullable),
                Scalar::primitive(data.time_duration, NonNullable),
            ],
        )
    }
}

impl SpanBuilder for Builder {
    type Output = StructArray;

    fn size(&self) -> usize {
        self.size
    }

    fn append(&mut self, data: Vec<crate::schema::SpanData>) -> bool {
        self.size += data.len();

        for data in data {
            self.builder
                .append_value(self.to_scalar(data).as_struct())
                .expect("append.struct.ok");
        }

        self.size >= self.threshold
    }

    fn build(&mut self) -> Self::Output {
        self.size = 0;
        self.builder.finish_into_struct()
    }
}
