use std::path::Path;

use arrow::array::{BooleanArray, Datum, RecordBatch};
use arrow::error::ArrowError;
use parquet::arrow::ProjectionMask;
use parquet::arrow::ArrowSchemaConverter;
use parquet::arrow::arrow_reader::{ArrowPredicate, RowFilter};
use parquet::arrow::arrow_reader::{ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder};
use parquet::schema::types::SchemaDescriptor;

use std::sync::Arc;

pub trait CustomFilter: ArrowPredicate + Sync {
    fn eval(&self, batch: &RecordBatch) -> Result<BooleanArray, ArrowError>;
}

#[derive(Clone)]
pub struct Boolean {
    mask: ProjectionMask,
    filters: Vec<Arc<dyn CustomFilter>>,
    function: fn(&BooleanArray, &BooleanArray) -> Result<BooleanArray, ArrowError>,
}

impl Boolean {
    fn mask(filters: &[Arc<dyn CustomFilter>]) -> ProjectionMask {
        assert!(!filters.is_empty());

        let mut mask = filters[0].projection().clone();

        for i in 1..filters.len() {
            mask.union(filters[i].projection()); 
        }

        mask
    }

    pub fn and(filters: Vec<Arc<dyn CustomFilter>>) -> Self {
        assert!(!filters.is_empty());

        Self {
            mask: Self::mask(filters.as_slice()),
            filters,
            function: arrow::compute::kernels::boolean::and,
        }
    }
}

impl CustomFilter for Boolean {
    fn eval(&self, batch: &RecordBatch) -> Result<BooleanArray, ArrowError> {
        let mut iter = self.filters.iter();
        let mut result = iter.next().unwrap().eval(&batch)?;

        for filter in iter {
            result = (self.function)(&result, &filter.eval(&batch)?)?;
        }

        Ok(result)
    }
}

impl ArrowPredicate for Boolean {
    fn projection(&self) -> &ProjectionMask {
        &self.mask
    }

    fn evaluate(&mut self, batch: RecordBatch) -> Result<BooleanArray, ArrowError> {
        self.eval(&batch)
    }
}

#[derive(Clone)]
pub struct Filter {
    mask: ProjectionMask,
    col_name: Arc<str>,
    value: Arc<dyn Datum + Sync + Send>,
    function: fn(&dyn Datum, &dyn Datum) -> Result<BooleanArray, ArrowError>,
}

impl Filter {
    pub fn new_u64(schema: &SchemaDescriptor, column: &str, value: u64) -> Self {
        use arrow::array::UInt64Array;

        Self {
            mask: ProjectionMask::columns(schema, [column]),
            col_name: Arc::from(column),
            value: Arc::new(UInt64Array::new_scalar(value)),
            function: arrow::compute::kernels::cmp::eq,
        }
    }

    pub fn new_str(schema: &SchemaDescriptor, column: &str, value: &str) -> Self {
        use arrow::array::StringViewArray;

        Self {
            mask: ProjectionMask::columns(schema, [column]),
            col_name: Arc::from(column),
            value: Arc::new(StringViewArray::new_scalar(value)),
            function: arrow::compute::kernels::cmp::eq,
        }
    }

    pub fn gte(mut self) -> Self {
        self.function = arrow::compute::kernels::cmp::gt_eq;
        self
    }

    pub fn lte(mut self) -> Self {
        self.function = arrow::compute::kernels::cmp::lt_eq;
        self
    }

    pub fn starts_with(mut self) -> Self {
        self.function = arrow::compute::kernels::comparison::starts_with;
        self
    }

    pub fn contains(mut self) -> Self {
        self.function = arrow::compute::kernels::comparison::contains;
        self
    }
}

impl CustomFilter for Filter {
    fn eval(&self, batch: &RecordBatch) -> Result<BooleanArray, ArrowError> {
        let col = batch.column_by_name(&self.col_name).expect(&format!("col.exists {}", self.col_name));
        
        (self.function)(col, &*self.value)
    }
}

impl ArrowPredicate for Filter {
    fn projection(&self) -> &ProjectionMask {
        &self.mask
    }

    fn evaluate(&mut self, batch: RecordBatch) -> Result<BooleanArray, ArrowError> {
        self.eval(&batch)
    }
}

async fn read_arrow_file(
    path: Box<Path>,
    projection: ProjectionMask,
    filter: RowFilter,
    limit: Option<usize>,
) -> ParquetRecordBatchReader {
    tokio::task::spawn_blocking(move || {
        tracing::info!(file = ?path, "Reading");
        let file = std::fs::File::open(path).expect("file.open");
        let builder = ParquetRecordBatchReaderBuilder::try_new(file).expect("builder.new");
        let mut builder = builder
            .with_projection(projection)
            .with_row_filter(filter);

        if let Some(limit) = limit {
            builder = builder.with_limit(limit);
        }

        builder
            .build()
            .expect("builder.build")

    }).await.unwrap()
}

pub struct Read<T> {
    select: ProjectionMask,
    filter: Vec<Box<T>>,
    files: Vec<Box<Path>>,
    limit: Option<usize>,
    index: usize,
    reader: Option<ParquetRecordBatchReader>,
}

impl<T> Read<T> {
    pub fn new<'a>(
        select: impl IntoIterator<Item = &'a str>,
        make_filter: impl Fn(&SchemaDescriptor) -> Vec<Box<T>>,
        files: Vec<Box<Path>>,
    ) -> Self {
        let schema = ArrowSchemaConverter::new().convert(&crate::schema::SCHEMA).unwrap();
        let select = ProjectionMask::columns(&schema, select.into_iter());
        let filter = make_filter(&schema);

        Self {
            select,
            filter,
            files,
            limit: None,
            index: 0,
            reader: None,
        }
    }
}

impl<T> Read<T>
where
    T: ArrowPredicate + Clone
{
    async fn init_reader(&mut self) {
        assert!(self.reader.is_none());
        let file = &self.files[self.index];

        self.reader.replace(read_arrow_file(
            file.clone(),
            self.select.clone(),
            RowFilter::new(self.filter.iter().map(|v| v.clone() as Box<dyn ArrowPredicate>).collect()),
            self.limit.clone(),
        ).await);
    }

    pub async fn next_batch(&mut self) -> Option<RecordBatch> {
        tracing::info!(idx = self.index, len = self.files.len(), "next_batch");

        while self.index < self.files.len() {
            if self.reader.is_none() {
                self.init_reader().await;
            }

            let next = self.reader.as_mut().expect("reader.exists").next();

            if let None = next {
                self.index += 1;
                self.reader = None;
                continue;
            }

            return Some(next.unwrap().expect("recordbatch.ok"));
        }

        None
    }
}

