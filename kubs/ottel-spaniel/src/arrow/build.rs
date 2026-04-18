use arrow::array::*;

use crate::SpanBuilder;
use crate::schema::*;

struct BatchBuilders {
    trace_id: FixedSizeListBuilder<UInt8Builder>,
    span_id: Int64Builder,
    parent_span_id: Int64Builder,
    name: StringViewBuilder,
    kind: Int32Builder,
    status_code: Int32Builder,
    status_message: StringViewBuilder,
    time_start: UInt64Builder,
    time_end: UInt64Builder,
    time_duration: UInt64Builder,
}

impl BatchBuilders {
    fn new(capacity: usize) -> Self {
        use arrow::datatypes::{DataType, Field};

        let trace_id = FixedSizeListBuilder::with_capacity(UInt8Builder::new(), 16, capacity)
            .with_field(Field::new_list_field(DataType::UInt8, false));
        let span_id = Int64Builder::with_capacity(capacity);
        let parent_span_id = Int64Builder::with_capacity(capacity);

        let name = StringViewBuilder::with_capacity(capacity);
        let kind = Int32Builder::with_capacity(capacity);

        let status_code = Int32Builder::with_capacity(capacity);
        let status_message = StringViewBuilder::with_capacity(capacity);

        let time_start = UInt64Builder::with_capacity(capacity);
        let time_end = UInt64Builder::with_capacity(capacity);
        let time_duration = UInt64Builder::with_capacity(capacity);

        Self {
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

    fn append(&mut self, data: &SpanData) {
        self.trace_id
            .values()
            .append_values(&data.trace_id, &[true; 16]);
        self.trace_id.append(true);

        self.span_id.append_value(i64::from_be_bytes(data.span_id));
        self.parent_span_id
            .append_option(data.parent_span_id.map(i64::from_be_bytes));

        self.name.append_value(&data.name);
        self.kind.append_value(data.kind);

        self.status_code.append_option(data.status_code);
        self.status_message
            .append_option(data.status_message.as_ref());

        self.time_start.append_value(data.time_start);
        self.time_end.append_value(data.time_end);
        self.time_duration.append_value(data.time_duration);
    }

    fn build(&mut self) -> Result<RecordBatch, arrow::error::ArrowError> {
        use std::sync::Arc;

        let cols: Vec<Arc<dyn Array>> = vec![
            Arc::new(self.trace_id.finish()),
            Arc::new(self.span_id.finish()),
            Arc::new(self.parent_span_id.finish()),
            Arc::new(self.name.finish()),
            Arc::new(self.kind.finish()),
            Arc::new(self.status_code.finish()),
            Arc::new(self.status_message.finish()),
            Arc::new(self.time_start.finish()),
            Arc::new(self.time_end.finish()),
            Arc::new(self.time_duration.finish()),
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

    fn append(&mut self, data: Vec<crate::schema::SpanData>) -> bool {
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
