use vortex::array::IntoArray;
use vortex::array::arrays::{Struct, StructArray};
use vortex::dtype::DType;
use vortex::file::BlockingWriter;
use vortex::file::WriteOptionsSessionExt;
use vortex::io::runtime::current::CurrentThreadRuntime;
use vortex::session::VortexSession;

use crate::{SpanWriter, Stats};

pub struct Writer<'a> {
    file_id: usize,
    stats: Stats,
    dtype: DType,
    writes: usize,
    threshold: usize,
    session: &'a VortexSession,
    runtime: &'a CurrentThreadRuntime,
    writer: Option<BlockingWriter<'a, 'a, CurrentThreadRuntime>>,
}

impl<'a> Writer<'a> {
    const DIR: &'static str = "data-vortex";
    const PREF: &'static str = "spaniel-live-vortex-";

    fn init_file_id(&mut self) {
        self.file_id = crate::misc::get_next_file_id(Self::DIR, Self::PREF);
    }

    async fn create_new_writer(&mut self) {
        self.close_writer().await;

        let file_path = format!("{}/{}{}", Self::DIR, Self::PREF, self.file_id);

        let writer = self
            .session
            .write_options()
            .blocking(self.runtime)
            .writer(crate::misc::open_file(&file_path), self.dtype.clone());

        assert!(self.writer.replace(writer).is_none());

        self.stats.set_dirty_file(&file_path).await;
    }

    pub async fn close_writer(&mut self) {
        if let Some(writer) = self.writer.take() {
            let result = writer.finish().expect("writer.finish.ok");
            tracing::info!(len = result.footer().row_count(), "writer.finish");
            self.stats.end_dirty_file().await;
        }
    }

    async fn next_file(&mut self) {
        self.file_id += 1;
        self.writes = 0;
        self.create_new_writer().await;
    }

    pub fn new(
        session: &'a VortexSession,
        rt: &'a CurrentThreadRuntime,
        spans_per_file: usize,
    ) -> Writer<'a> {
        Self {
            file_id: 0,
            threshold: spans_per_file,
            dtype: super::build::create_struct_dtype(),
            session,
            runtime: rt,
            writes: 0,
            writer: None,
            stats: Stats::new(Self::DIR, Self::PREF),
        }
    }

    async fn write_data(&mut self, data: StructArray) {
        tracing::info!(len = data.len(), writes = self.writes, "writer.save");
        self.writes += data.len();
        self.writer
            .as_mut()
            .unwrap()
            .push(data.into_array())
            .expect("writer.push.ok");
    }
}

impl<'a> SpanWriter for Writer<'a> {
    type Input = StructArray;

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

        if data.len() + self.writes > self.threshold {
            let diff = self.threshold - self.writes;
            let first = data.slice(0..diff).expect("array.slice.ok");
            let second = data.slice(diff..data.len()).expect("array.slice.ok");

            self.write_data(first.downcast::<Struct>()).await;
            self.next_file().await;
            self.write_data(second.downcast::<Struct>()).await;
            return;
        }

        self.write_data(data).await;
    }

    async fn suspend(&mut self) {
        self.close_writer().await;
    }

    async fn finish(mut self) {
        self.close_writer().await;
    }
}
