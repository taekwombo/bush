use std::io::BufWriter;
use std::fs::{self, File};

use arrow::array::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;

use crate::schema::SCHEMA;

pub struct Writer {
    writer: ArrowWriter<BufWriter<File>>,
    file_id: usize,
    writes: usize,
}

impl Writer {
    fn path() -> &'static str {
        "spaniel-live-arrow-"
    }

    fn dir() -> &'static str {
        "data-arrow"
    }

    fn get_file_id() -> usize {
        let Ok(meta) = fs::metadata(Self::dir()) else {
            fs::create_dir(Self::dir()).expect("dir.create.ok");
            return 0;
        };

        assert!(meta.is_dir());

        let files = fs::read_dir(Self::dir()).expect("dir.read.ok");
        let mut max: usize = 0;
        let mut count: usize = 0;

        for file in files.map(Result::unwrap) {
            let name = file.file_name().into_string().expect("osstring.convert");

            if name.len() <= Self::path().len() {
                continue;
            }
            
            let suffix = &name[Self::path().len()..];
            let num: usize = suffix.parse().expect("parse.ok");

            if max < num {
                max = num;
            }

            count += 1;
        }

        if count == 0 { 0 } else { max + 1 }
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

    pub fn new() -> Self {
        let id = Self::get_file_id();
        let file = Self::open_file(id);
        let file = BufWriter::new(file);

        Self {
            writer: ArrowWriter::try_new(file, SCHEMA.clone(), None).expect("writer.create.ok"),
            file_id: id,
            writes: 0,
        }
    }

    pub fn save(&mut self, data: RecordBatch) {
        self.writer.write(&data).expect("writer.write.ok");
        self.writer.flush().expect("writer.flush.ok");
        self.writer.sync().expect("writer.sync.ok");
        self.writes += 1;

        if self.writes >= 10 {
            tracing::info!(writes = self.writes, "reached write treshold");
            self.next_file();
        }
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
