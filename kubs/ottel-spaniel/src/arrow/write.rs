use std::io::BufWriter;
use std::fs::File;

use arrow::array::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;

use crate::schema::SCHEMA;

pub struct Writer {
    writer: ArrowWriter<BufWriter<File>>,
    file_id: usize,
    writes: usize,
    threshold: usize,
}

impl Writer {
    fn path() -> &'static str {
        "spaniel-live-arrow-"
    }

    fn dir() -> &'static str {
        "data-arrow"
    }

    fn open_file(id: usize) -> File {
        let file_name = format!("{}/{}{}", Self::dir(), Self::path(), id);
        tracing::info!(file = file_name, "Opening file");

        File::options()
            .write(true)
            .create_new(true)
            .open(file_name)
            .expect("writer.file.open")
    }

    pub fn new(spans_per_file: usize) -> Self {
        let id = crate::misc::get_next_file_id(Self::dir(), Self::path());
        let file = Self::open_file(id);
        let file = BufWriter::new(file);

        Self {
            writer: ArrowWriter::try_new(file, SCHEMA.clone(), None).expect("writer.create.ok"),
            file_id: id,
            writes: 0,
            threshold: spans_per_file,
        }
    }

    pub fn save(&mut self, data: RecordBatch) {
        if data.num_rows() + self.writes > self.threshold {
            let diff = self.threshold - self.writes;
            let first = data.slice(0, diff);
            let second = data.slice(diff, data.num_rows() - diff);
            self.save(first);
            self.next_file();
            self.save(second);
            return;
        }

        self.writer.write(&data).expect("writer.write.ok");
        self.writer.flush().expect("writer.flush.ok");
        self.writer.sync().expect("writer.sync.ok");
        self.writes += data.num_rows();
    }

    fn next_file(&mut self) {
        self.finish();
        
        self.file_id += 1;
        self.writes = 0;

        let file = Self::open_file(self.file_id);
        let file = BufWriter::new(file);
        self.writer = ArrowWriter::try_new(file, SCHEMA.clone(), None).expect("writer.create.ok");
    }

    pub fn finish(&mut self) {
        self.writer.finish().expect("writer.finish.ok");
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        assert!(self.writer.in_progress_rows() == 0);
    }
}
