use arrow::array::*;

pub struct BatchBuilders {
    trace_id: FixedSizeListBuilder<UInt8Builder>,
    span_id: Int64Builder,
    parent_span_id: Int64Builder,
    name: StringViewBuilder,
    kind: Int32Builder,
    status_code: Int32Builder,
    status_message: StringViewBuilder,
}

impl BatchBuilders {
    fn new(capacity: usize) -> Self {
        use arrow::datatypes::{Field, DataType};

        let trace_id = FixedSizeListBuilder::with_capacity(UInt8Builder::new(), 16, capacity)
            .with_field(Field::new_list_field(DataType::UInt8, false));
        let span_id = Int64Builder::with_capacity(capacity);
        let parent_span_id = Int64Builder::with_capacity(capacity);

        let name = StringViewBuilder::with_capacity(capacity);
        let kind = Int32Builder::with_capacity(capacity);

        let status_code = Int32Builder::with_capacity(capacity);
        let status_message = StringViewBuilder::with_capacity(capacity);

        Self {
            trace_id,
            span_id,
            parent_span_id,
            name,
            kind,
            status_code,
            status_message,
        }
    }

    fn append(&mut self, data: &SpanData) {
        self.trace_id.values().append_values(&data.trace_id, &[true; 16]);
        self.trace_id.append(true);

        self.span_id.append_value(i64::from_be_bytes(data.span_id));
        self.parent_span_id.append_option(data.parent_span_id.map(i64::from_be_bytes));

        self.name.append_value(&data.name);
        self.kind.append_value(data.kind);

        self.status_code.append_option(data.status_code);
        self.status_message.append_option(data.status_message.as_ref());
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
        ];

        RecordBatch::try_new(crate::schema::SCHEMA.clone(), cols)
    }
}

pub struct SpanData {
    pub trace_id: [u8; 16],
    pub span_id: [u8; 8],
    pub parent_span_id: Option<[u8; 8]>,
    pub name: String,
    pub kind: i32,
    pub status_code: Option<i32>,
    pub status_message: Option<String>,
}

pub type SpanBatch = Vec<SpanData>;

pub struct BatchWriter {
    builders: BatchBuilders,
    pub written: usize,
    pub threshold: usize,
}

impl BatchWriter {
    pub fn new(threshold: usize, capacity: usize) -> Self {
        Self {
            builders: BatchBuilders::new(capacity),
            written: 0,
            threshold,
        }
    }

    // Returns bool indicating whether data should be batched.
    pub fn append(&mut self, data: &[SpanData]) -> bool {
        for data in data {
            self.builders.append(data);
        }

        self.written += data.len();
        self.written >= self.threshold
    }

    pub fn build(&mut self) -> Result<RecordBatch, arrow::error::ArrowError> {
        self.written = 0;

        self.builders.build()
    }
}
