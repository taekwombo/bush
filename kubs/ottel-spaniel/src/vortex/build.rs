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

    tracing::info!(out_len = trace_id.len(), "creating trace_id");

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

// impl BatchBuilders {
//     fn new(capacity: usize) -> Self {
//         let trace_id = FixedSizeListBuilder::with_capacity(
//             Arc::new(DType::Primitive(PType::U8, Nullable)),
//             16,
//             NonNullable,
//             capacity,
//         );
//         let span_id = PrimitiveBuilder::with_capacity(NonNullable, capacity);
//         let parent_span_id = PrimitiveBuilder::with_capacity(Nullable, capacity);
//
//         let name = VarBinViewBuilder::with_capacity(DType::Utf8(NonNullable), capacity);
//         let kind = PrimitiveBuilder::with_capacity(NonNullable, capacity);
//
//         let status_code = PrimitiveBuilder::with_capacity(Nullable, capacity);
//         let status_message = VarBinViewBuilder::with_capacity(DType::Utf8(Nullable), capacity);
//
//         let time_start = PrimitiveBuilder::with_capacity(NonNullable, capacity);
//         let time_end = PrimitiveBuilder::with_capacity(NonNullable, capacity);
//         let time_duration = PrimitiveBuilder::with_capacity(NonNullable, capacity);
//
//         Self {
//             trace_id,
//             span_id,
//             parent_span_id,
//             name,
//             kind,
//             status_code,
//             status_message,
//             time_start,
//             time_end,
//             time_duration,
//         }
//     }
//
//     fn append(&mut self, data: &SpanData) {
//         self.trace_id.append_array_as_list(
//             PrimitiveArray::from_iter(data.trace_id).as_array(),
//         ).expect("trace_id.append.ok");
//         self.span_id.append_value(i64::from_be_bytes(data.span_id));
//         match data.parent_span_id {
//             Some(id) => self.parent_span_id.append_value(i64::from_be_bytes(id)),
//             None => self.parent_span_id.append_null(),
//         }
//
//         self.name.append_value(&data.name);
//         self.kind.append_value(data.kind);
//
//         match data.status_code {
//             Some(id) => self.status_code.append_value(id),
//             None => self.status_code.append_null(),
//         }
//
//         match data.status_message {
//             Some(ref s) => self.status_message.append_value(s),
//             None => self.status_message.append_null(),
//         }
//
//         self.time_start.append_value(data.time_start);
//         self.time_end.append_value(data.time_end);
//         self.time_duration.append_value(data.time_duration);
//     }
//
//     fn build(&mut self) -> Arrays {
//         Arrays {
//             trace_id: self.trace_id.finish_into_fixed_size_list(),
//             span_id: self.span_id.finish_into_primitive(),
//             parent_span_id: self.parent_span_id.finish_into_primitive(),
//             name: self.name.finish_into_varbinview(),
//             kind: self.kind.finish_into_primitive(),
//             status_code: self.status_code.finish_into_primitive(),
//             status_message: self.status_message.finish_into_varbinview(),
//             time_start: self.time_start.finish_into_primitive(),
//             time_end: self.time_end.finish_into_primitive(),
//             time_duration: self.time_duration.finish_into_primitive(),
//         }
//     }
// }

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
            tracing::info!("append.pre");
            self.builder.append_value(to_scalar(data).as_struct()).expect("append.struct.ok");
            tracing::info!("append.ok");
        }

        self.size >= self.threshold
    }

    pub fn build(&mut self) -> StructArray {
        self.size = 0;
        self.builder.finish_into_struct()
    }
}
