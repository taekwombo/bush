use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use arrow::array::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;

use crate::schema::SCHEMA;
use crate::{SpanWriter, Stats};

pub struct Writer {
    file_id: usize,
    writer: Option<ArrowWriter<BufWriter<File>>>,
    stats: Stats,
    /// Number of written spans to current file.
    writes: usize,
    /// Maximum number of Spans per file.
    threshold: usize,
}

impl Writer {
    const DIR: &str = "data-arrow";
    const PREF: &str = "spaniel-live-arrow-";

    fn open_file(path: impl AsRef<Path>) -> BufWriter<File> {
        tracing::info!(file = ?path.as_ref().as_os_str(), "Opening file");

        let file = File::options()
            .write(true)
            .create_new(true)
            .open(path)
            .expect("writer.file.open");

        BufWriter::new(file)
    }

    fn init_file_id(&mut self) {
        self.file_id = crate::misc::get_next_file_id(Self::DIR, Self::PREF);
    }

    async fn next_file(&mut self) {
        self.file_id += 1;
        self.writes = 0;
        self.create_new_writer().await;
    }

    async fn create_new_writer(&mut self) {
        let file_path = format!("{}/{}{}", Self::DIR, Self::PREF, self.file_id);

        #[allow(clippy::borrow_interior_mutable_const)]
        let writer = ArrowWriter::try_new(
            Self::open_file(&file_path),
            SCHEMA.clone(),
            None,
        ).expect("arrow-writer.create");

        let old = self.writer.replace(writer);

        if let Some(writer) = old {
            writer.close().expect("writer.close");
        };

        self.stats.set_dirty_file(&file_path).await;
    }

    pub fn new(spans_per_file: usize) -> Self {
        Self {
            file_id: 0,
            writer: None,
            stats: Stats::default(),
            writes: 0,
            threshold: spans_per_file,
        }
    }

    fn write_data(&mut self, data: RecordBatch) {
        let Some(writer) = self.writer.as_mut() else {
            unreachable!();
        };

        tracing::info!(len = data.num_rows(), writes = self.writes, "writer.save");
        self.writes += data.num_rows();
        writer.write(&data).expect("write.ok");
    }
}

impl SpanWriter for Writer {
    type Input = RecordBatch;

    fn is_dirty(&self) -> bool {
        self.writes > 0
    }

    fn stats(&self) -> &Stats {
        &self.stats
    }

    async fn write(&mut self, data: Self::Input) {
        if self.writer.is_none() {
            self.init_file_id();
            self.create_new_writer().await;
        }

        if data.num_rows() > (self.threshold - self.writes) {
            let diff = self.threshold - self.writes;
            let first = data.slice(0, diff);
            let second = data.slice(diff, data.num_rows() - diff);
            self.write_data(first);
            self.next_file().await;
            self.write_data(second);
            return;
        }

        self.write_data(data);
    }

    async fn suspend(&mut self) {
        if let Some(writer) = self.writer.take() {
            writer.close().expect("writer.close");
        }
    }

    async fn finish(self) {
        let Some(writer) = self.writer else {
            return;
        };

        writer.close().expect("writer.close");
    }
}
