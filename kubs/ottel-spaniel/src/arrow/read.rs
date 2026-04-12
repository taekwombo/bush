use arrow::array::{BooleanArray, Datum, RecordBatch};
use arrow::error::ArrowError;
use parquet::arrow::ProjectionMask;
use parquet::arrow::arrow_reader::ArrowPredicate;
use parquet::schema::types::SchemaDescriptor;

use std::sync::Arc;

pub struct Filter {
    mask: ProjectionMask,
    col_name: Arc<str>,
    value: Arc<dyn Datum + Sync + Send>,
    function: fn(&dyn Datum, &dyn Datum) -> Result<BooleanArray, ArrowError>,
}

impl Filter {
    pub fn new_str(schema: &SchemaDescriptor, column: &str, value: &str) -> Self {
        use arrow::array::StringViewArray;

        Self {
            mask: ProjectionMask::columns(schema, [column]),
            col_name: Arc::from(column),
            value: Arc::new(StringViewArray::new_scalar(value)),
            function: arrow::compute::kernels::cmp::eq,
        }
    }

    pub fn starts_with(&mut self) {
        self.function = arrow::compute::kernels::comparison::starts_with;
    }
}

impl ArrowPredicate for Filter {
    fn projection(&self) -> &ProjectionMask {
        &self.mask
    }

    fn evaluate(&mut self, batch: RecordBatch) -> Result<BooleanArray, ArrowError> {
        let col = batch.column_by_name(&self.col_name).expect("col.exists");

        (self.function)(col, &*self.value)
    }
}
