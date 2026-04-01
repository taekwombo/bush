pub mod builder;
pub mod convert;
pub mod schema;

use builder::SpanBatch;

use std::io::BufWriter;
use std::fs::File;
use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::{mpsc, oneshot};
use parquet::arrow::arrow_writer::ArrowWriter;

pub type Message = (oneshot::Sender<()>, SpanBatch);

pub struct BobbySinker {
    batch_sink: mpsc::Sender<Message>,
}

impl BobbySinker {
    fn new(batch_sink: mpsc::Sender<Message>) -> Self {
        Self { batch_sink }
    }

    pub async fn write(&self, batch: SpanBatch) -> () {
        let (tx, rx) = oneshot::channel::<()>();

        self.batch_sink.send((tx, batch)).await.expect("sink.batch_sink.alive");
        rx.await.expect("sink.oneshot.ok");
    }
}

#[allow(dead_code)]
pub struct Writchester {
    waitlist: Vec<oneshot::Sender<()>>,
    batch_writer: builder::BatchWriter,
    writer: ArrowWriter<BufWriter<File>>,
    rx: mpsc::Receiver<Message>,

    file_limit: usize,
    written: usize,
}

impl Writchester {
    fn get_pathname() -> &'static str {
        "./spaniel-live"
    }

    fn create_live() -> BufWriter<File> {
        BufWriter::new(
            File::options()
                .write(true)
                .create_new(true)
                .open(Self::get_pathname())
                .expect("writchester.create_live.ok")
        )
    }

    fn new(build_threshold: usize, rx: mpsc::Receiver<Message>) -> Self {
        let capacity = build_threshold * 2;

        Self {
            batch_writer: builder::BatchWriter::new(build_threshold, capacity),
            waitlist: Vec::with_capacity(capacity),
            writer: ArrowWriter::try_new(Self::create_live(), crate::schema::SCHEMA.clone(), None).expect("writer.create"),
            rx,
            file_limit: 20000,
            written: 0,
        }
    }

    async fn run(mut self) -> () {
        'forever: loop {
            tokio::select! {
                message = self.rx.recv() => {
                    let Some((waiting, batch)) = message else {
                        break 'forever;
                    };

                    self.waitlist.push(waiting);

                    if self.batch_writer.append(&batch) {
                        self.save_batch();
                    }
                },
                _ = sleep(Duration::from_millis(1000)) => {
                    self.save_batch();
                }
            }
        }

        self.save_batch();
        self.writer.finish().expect("ok");
    }

    fn save_batch(&mut self) -> () {
        if self.batch_writer.written == 0 {
            assert!(self.waitlist.is_empty());
            return;
        }

        let batch = self.batch_writer.build().expect("writer.build.ok");

        self.written += batch.num_rows();
        // println!("[{}] {} -- {} : {} @ {}", self.writer.memory_size(), self.written, self.writer.bytes_written(), self.writer.in_progress_rows(), self.writer.in_progress_size());
        self.writer.write(&batch).expect("writer.write.ok");
        self.writer.flush().expect("writer.sync.ok");
        self.writer.sync().expect("writer.sync.ok");

        self.confirm();
    }

    fn confirm(&mut self) {
        for s in self.waitlist.drain(..) {
            s.send(()).expect("waitlist.oneshot.alive");
        }
        assert!(self.waitlist.len() == 0);
    }
}

impl Drop for Writchester {
    fn drop(&mut self) {
        assert!(self.waitlist.is_empty());
        assert!(self.rx.is_empty());
        assert!(self.rx.is_closed());

        assert!(self.batch_writer.written == 0);
        assert!(self.writer.in_progress_rows() == 0);
    }
}

#[cfg(feature = "free-for-all")]
pub fn create_writer(channel_size: usize) -> (BobbySinker, tokio::task::JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<Message>(channel_size);
    let sinker = BobbySinker::new(tx);
    let writer = Writchester::new(channel_size, rx);

    let handle = tokio::spawn(async move {
        writer.run().await;
    });

    (sinker, handle)
}

#[cfg(not(feature = "free-for-all"))]
pub fn start<T>(channel_size: usize, t: impl FnOnce(BobbySinker) -> T + Send + 'static)
where
    T: Future<Output = ()>,
    T: Send + 'static,
{
    use std::sync::{Arc, Mutex};

    use core_affinity::*;
    use tokio::runtime::{Builder, LocalOptions};

    let (sinker, writer) = {
        let (tx, rx) = mpsc::channel::<Message>(channel_size);

        (BobbySinker::new(tx), Writchester::new(channel_size, rx))
    };

    let mut ids = get_core_ids().expect("get_core_ids.ok");
    let sink_id = ids.pop().unwrap();

    assert!(!ids.is_empty());
    let workers = ids.len().min(4);
    let ids = Arc::new(Mutex::new(ids));

    let sink_rt = Builder::new_current_thread()
        .on_thread_start(move || {
            set_for_current(sink_id);
        })
        .enable_all()
        .build_local(LocalOptions::default())
        .expect("sink.rt.ok");

    let server_rt = Builder::new_multi_thread()
        .worker_threads(workers)
        .on_thread_start(move || {
            let id = ids.clone().lock().expect("mutex.ok").pop().expect("pop.ok");
            set_for_current(id);
        })
        .enable_all()
        .build()
        .expect("server.rt.ok");

    server_rt.spawn(t(sinker));

    sink_rt.block_on(async move {
        writer.run().await;
    });
}
