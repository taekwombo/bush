use std::io::BufWriter;
use std::fs::File;

use vortex::array::arrays::{StructArray, Struct};
use vortex::file::BlockingWriter;
use vortex::file::WriteOptionsSessionExt;
use vortex::io::runtime::current::CurrentThreadRuntime;
use vortex::session::VortexSession;

pub struct Writer<'a> {
    file_id: usize,
    writes: usize,
    threshold: usize,
    session: &'a VortexSession,
    runtime: &'a CurrentThreadRuntime,
    writer: BlockingWriter<'a, 'a, CurrentThreadRuntime>,
}

impl Writer<'_> {
    fn path() -> &'static str {
        "spaniel-live-vortex-"
    }

    fn dir() -> &'static str {
        "data-vortex"
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

    pub fn save(&mut self, data: StructArray) {
        use vortex::array::IntoArray;

        if data.len() + self.writes > self.threshold {
            let diff = self.threshold - self.writes;
            let first = data.slice(0..diff).expect("array.slice.ok");
            let second = data.slice(diff..data.len()).expect("array.slice.ok");

            self.save(first.downcast::<Struct>());
            self.next_file();
            self.save(second.downcast::<Struct>());
            return;
        }

        tracing::info!(len = data.len(), writes = self.writes, "writer.save");
        self.writes += data.len();
        self.writer.push(data.into_array()).expect("writer.push.ok");
    }

    pub fn finish(self) {
        let result = self.writer.finish().expect("writer.finish.ok");
        tracing::info!(len = result.footer().row_count(), "writer.finish");
    }

    fn next_file(&mut self) {
        self.file_id += 1;
        self.writes = 0;

        let file = Self::open_file(self.file_id);
        let file = BufWriter::new(file);

        let mut writer = self.session
            .write_options()
            .blocking(self.runtime)
            .writer(file, super::build::STRUCT.clone());

        std::mem::swap(&mut writer, &mut self.writer);

        writer.finish().expect("writer.finish.ok");
    }
}

impl<'a> Writer<'a> {
    pub fn new(session: &'a VortexSession, rt: &'a CurrentThreadRuntime, spans_per_file: usize) -> Writer<'a> {

        let id = crate::misc::get_next_file_id(Self::dir(), Self::path());
        let file = Self::open_file(id);
        let file = BufWriter::new(file);

        Self {
            file_id: id,
            threshold: spans_per_file,
            session,
            runtime: rt,
            writes: 0,
            writer: session.write_options().blocking(rt).writer(file, super::build::STRUCT.clone()),
        }
    }
}
