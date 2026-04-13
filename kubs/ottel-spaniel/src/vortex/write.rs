use std::io::BufWriter;
use std::fs::{self, File};

use vortex::file::BlockingWriter;
use vortex::io::runtime::current::CurrentThreadRuntime;
use vortex::session::VortexSession;

pub struct Writer<'a> {
    writes: usize,
    writer: BlockingWriter<'a, 'a, CurrentThreadRuntime>,
}

impl Writer<'_> {
    fn path() -> &'static str {
        "spaniel-live-vortex-"
    }

    fn dir() -> &'static str {
        "data-vortex"
    }

    fn get_file_id() -> usize {
        let Ok(meta) = fs::metadata(Self::dir()) else {
            fs::create_dir(Self::dir()).expect("dir.create.ok");
            return 0;
        };

        assert!(meta.is_dir());

        0
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

    pub fn save(&mut self, data: vortex::array::arrays::StructArray) {
        use vortex::array::IntoArray;

        self.writes += 1;
        self.writer.push(data.into_array()).expect("writer.push.ok");
    }

    pub fn finish(self) {
        let result = self.writer.finish().expect("writer.finish.ok");
        println!("{:?}", result.footer());
    }
}

impl<'a> Writer<'a> {
    pub fn new(session: &'a VortexSession, rt: &'a CurrentThreadRuntime) -> Writer<'a> {
        use vortex::file::WriteOptionsSessionExt;

        let file = Self::open_file(Self::get_file_id());
        let file = BufWriter::new(file);

        Self {
            writes: 0,
            writer: session.write_options().blocking(rt).writer(file, super::build::STRUCT.clone()),
        }
    }
}
