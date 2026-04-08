use std::io::BufWriter;
use std::fs::File;

use super::build::Builder;
use parquet::arrow::arrow_writer::ArrowWriter;

use crate::schema::*;

pub struct Writer {
    pub builder: Builder,
    writer: ArrowWriter<BufWriter<File>>,
}

impl Writer {
    fn path() -> &'static str {
        "spaniel-live-arrow"
    }

    pub fn new(threshold: usize) -> Self {
        let file = File::options()
            .write(true)
            .create_new(true)
            .open(Self::path())
            .expect("writer.file.open");
        let file = BufWriter::new(file);

        Self {
            builder: Builder::new(threshold, threshold * 2),
            writer: ArrowWriter::try_new(file, SCHEMA.clone(), None).expect("writer.create.ok"),
        }
    }

    pub fn append(&mut self, data: &[SpanData]) -> bool {
        self.builder.append(data)
    }

    pub fn save(&mut self) {
        let data = self.builder.build();

        self.writer.write(&data).expect("writer.write.ok");
        self.writer.flush().expect("writer.flush.ok");
        self.writer.sync().expect("writer.sync.ok");
    }

    pub fn finish(&mut self) {
        self.writer.finish().expect("writer.finish.ok");
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        assert!(self.builder.size == 0);
        assert!(self.writer.in_progress_rows() == 0);
    }
}
