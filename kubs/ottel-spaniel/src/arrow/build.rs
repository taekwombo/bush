use arrow::array::*;

use super::{Attribute, SCHEMA, columns};
use crate::SpanBuilder;
use crate::SpanData;

struct BatchBuilders {
    trace_id: FixedSizeBinaryBuilder,
    span_id: Int64Builder,
    span_name: StringViewBuilder,
    span_kind: Int32Builder,
    parent_span_id: Int64Builder,
    status_code: Int32Builder,
    status_message: StringViewBuilder,
    time_start: UInt64Builder,
    time_end: UInt64Builder,
    time_duration: UInt64Builder,
    res_attr_name: ListBuilder<StringViewBuilder>,
    res_attr_ty: ListBuilder<Int8Builder>,
    res_attr_value: ListBuilder<BinaryViewBuilder>,
    span_attr_name: ListBuilder<StringViewBuilder>,
    span_attr_ty: ListBuilder<Int8Builder>,
    span_attr_value: ListBuilder<BinaryViewBuilder>,
}

impl BatchBuilders {
    fn new(capacity: usize) -> Self {
        let trace_id = FixedSizeBinaryBuilder::with_capacity(capacity, 16);
        let span_id = Int64Builder::with_capacity(capacity);
        let span_name = StringViewBuilder::with_capacity(capacity).with_deduplicate_strings();
        let span_kind = Int32Builder::with_capacity(capacity);

        let parent_span_id = Int64Builder::with_capacity(capacity);

        let status_code = Int32Builder::with_capacity(capacity);
        let status_message = StringViewBuilder::with_capacity(capacity).with_deduplicate_strings();

        let time_start = UInt64Builder::with_capacity(capacity);
        let time_end = UInt64Builder::with_capacity(capacity);
        let time_duration = UInt64Builder::with_capacity(capacity);

        let res_attr_name = ListBuilder::with_capacity(
            StringViewBuilder::new().with_deduplicate_strings(),
            capacity,
        )
        .with_field(columns::RES_ATTR_NAME.as_list_field());
        let res_attr_ty = ListBuilder::with_capacity(Int8Builder::new(), capacity)
            .with_field(columns::RES_ATTR_TYPE.as_list_field());
        let res_attr_value = ListBuilder::with_capacity(BinaryViewBuilder::new(), capacity)
            .with_field(columns::RES_ATTR_VALUE.as_list_field());

        let span_attr_name = ListBuilder::with_capacity(
            StringViewBuilder::new().with_deduplicate_strings(),
            capacity,
        )
        .with_field(columns::SPAN_ATTR_NAME.as_list_field());
        let span_attr_ty = ListBuilder::with_capacity(Int8Builder::new(), capacity)
            .with_field(columns::SPAN_ATTR_TYPE.as_list_field());
        let span_attr_value = ListBuilder::with_capacity(BinaryViewBuilder::new(), capacity)
            .with_field(columns::SPAN_ATTR_VALUE.as_list_field());

        Self {
            trace_id,
            span_id,
            span_name,
            span_kind,
            parent_span_id,
            status_code,
            status_message,
            time_start,
            time_end,
            time_duration,
            res_attr_name,
            res_attr_ty,
            res_attr_value,
            span_attr_name,
            span_attr_ty,
            span_attr_value,
        }
    }

    fn append(&mut self, data: &SpanData) {
        self.trace_id
            .append_value(data.trace_id)
            .expect("trace_id.append");

        self.span_id.append_value(i64::from_be_bytes(data.span_id));
        self.span_name.append_value(&data.name);
        self.span_kind.append_value(data.kind);

        self.parent_span_id
            .append_option(data.parent_span_id.map(i64::from_be_bytes));

        self.status_code.append_option(data.status_code);
        self.status_message
            .append_option(data.status_message.as_ref());

        self.time_start.append_value(data.time_start);
        self.time_end.append_value(data.time_end);
        self.time_duration.append_value(data.time_duration);

        if data.parent_span_id.is_some() {
            Attribute::append(
                &mut self.res_attr_name,
                &mut self.res_attr_ty,
                &mut self.res_attr_value,
                data.resource_attributes.as_ref(),
            );
            self.res_attr_name.append(true);
            self.res_attr_ty.append(true);
            self.res_attr_value.append(true);
        } else {
            self.res_attr_name.append(false);
            self.res_attr_ty.append(false);
            self.res_attr_value.append(false);
        }

        Attribute::append(
            &mut self.span_attr_name,
            &mut self.span_attr_ty,
            &mut self.span_attr_value,
            data.span_attributes.as_ref(),
        );
        self.span_attr_name.append(true);
        self.span_attr_ty.append(true);
        self.span_attr_value.append(true);
    }

    fn build(&mut self) -> Result<RecordBatch, arrow::error::ArrowError> {
        use std::sync::Arc;

        let cols: Vec<Arc<dyn Array>> = vec![
            Arc::new(self.trace_id.finish()),
            Arc::new(self.span_id.finish()),
            Arc::new(self.span_name.finish()),
            Arc::new(self.span_kind.finish()),
            Arc::new(self.parent_span_id.finish()),
            Arc::new(self.status_code.finish()),
            Arc::new(self.status_message.finish()),
            Arc::new(self.time_start.finish()),
            Arc::new(self.time_end.finish()),
            Arc::new(self.time_duration.finish()),
            Arc::new(self.res_attr_name.finish()),
            Arc::new(self.res_attr_ty.finish()),
            Arc::new(self.res_attr_value.finish()),
            Arc::new(self.span_attr_name.finish()),
            Arc::new(self.span_attr_ty.finish()),
            Arc::new(self.span_attr_value.finish()),
        ];

        #[allow(clippy::borrow_interior_mutable_const)]
        RecordBatch::try_new(SCHEMA.clone(), cols)
    }
}

pub struct Builder {
    builders: BatchBuilders,
    /// Number of spans written since last build.
    pub size: usize,
    /// Number of spans that should trigger build.
    pub threshold: usize,
}

impl Builder {
    pub fn new(threshold: usize, capacity: usize) -> Self {
        Self {
            builders: BatchBuilders::new(capacity),
            size: 0,
            threshold,
        }
    }
}

impl SpanBuilder for Builder {
    type Output = RecordBatch;

    fn append(&mut self, data: Vec<SpanData>) -> bool {
        for data in data.iter() {
            self.builders.append(data);
        }

        self.size += data.len();
        self.size >= self.threshold
    }

    fn build(&mut self) -> Self::Output {
        self.size = 0;

        self.builders.build().expect("builder.build")
    }

    fn size(&self) -> usize {
        self.size
    }
}
