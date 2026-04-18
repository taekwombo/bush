use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use vortex::io::runtime::current::CurrentThreadRuntime;
use vortex::session::VortexSession;

use tokio::sync::RwLock;
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub enum Format {
    /// Store the span data using Arrow/Parquet.
    Arrow,
    /// Store the span data using Vortex.
    Vortex {
        runtime: CurrentThreadRuntime,
        session: VortexSession,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Interval at which data buffered by Bulider should be passed down to Writer.
    pub flush_interval_millis: u64,
    /// Interval used after "suspend_after" intervals are completed with no new messages.
    pub suspend_interval_millis: u64,
    /// Suspend writer after N attempts to flush empty Builder buffer.
    pub suspend_after: u64,
    /// Capacity of the channel which accepts data to be written.
    pub sink_channel_size: usize,
    /// Capacity of the buffer which holds channels telling that data was written.
    pub request_waitlist_size: usize,
    /// Number of Spans per file.
    pub spans_per_file: usize,
    /// Number of Spans accepted by builder that should trigger write.
    pub builder_flush_threshold: usize,
    /// Default capacity of builder buffers.
    pub builder_capacity: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Stats {
    /// File currently used by the writer.
    dirty_file: Arc<RwLock<Option<Box<Path>>>>,
    /// Files available for reading.
    files: Arc<RwLock<Vec<Box<Path>>>>,
}

impl Stats {
    pub async fn append_files(&self, add: &[impl AsRef<Path>]) {
        let to_add: Vec<Box<Path>> = add
            .iter()
            .map(|v| v.as_ref().to_path_buf().into())
            .collect();

        self.files.write().await.extend_from_slice(&to_add);
    }

    pub async fn set_dirty_file(&self, path: impl AsRef<Path>) {
        let old = {
            let mut d = self.dirty_file.write().await;
            d.replace(path.as_ref().to_path_buf().into())
        };

        if let Some(old) = old {
            let mut files = self.files.write().await;
            files.push(old);
        }
    }
}

#[derive(Debug)]
pub struct Message {
    on_done: oneshot::Sender<()>,
    pub data: Vec<crate::schema::SpanData>,
}

#[derive(Clone, Debug)]
pub struct Sink {
    sender: mpsc::Sender<Message>,
}

impl Sink {
    /// Sends [crate::schema::SpanData] to be written to storage.
    pub async fn send(&self, data: Vec<crate::schema::SpanData>) {
        let (send, recv) = oneshot::channel::<()>();

        self.sender
            .send(Message {
                data,
                on_done: send,
            })
            .await
            .expect("sender.alive");

        recv.await.expect("oneshot.recv");
    }
}

pub trait SpanWriter {
    type Input;

    fn is_dirty(&self) -> bool;
    fn stats(&self) -> &Stats;
    fn write(&mut self, data: Self::Input) -> impl Future<Output = ()>;
    fn suspend(&mut self) -> impl Future<Output = ()>;
    fn finish(self) -> impl Future<Output = ()>;
}

pub trait SpanBuilder {
    type Output;

    fn size(&self) -> usize;
    fn append(&mut self, data: Vec<crate::schema::SpanData>) -> bool;
    fn build(&mut self) -> Self::Output;
}

async fn run_writer<T, W, B>(
    mut writer: W,
    mut builder: B,
    mut rx: mpsc::Receiver<Message>,
    options: Options,
) where
    W: SpanWriter<Input = T>,
    B: SpanBuilder<Output = T>,
{
    let mut waitlist = Vec::with_capacity(options.request_waitlist_size);

    let done = |list: &mut Vec<oneshot::Sender<()>>| {
        for i in list.drain(..) {
            i.send(()).expect("oneshot.send");
        }
        assert!(list.is_empty());
    };

    let mut times_since_last_action = 1;

    'forever: loop {
        let interval = if times_since_last_action >= options.suspend_after {
            options.suspend_interval_millis
        } else {
            options.flush_interval_millis
        };

        tokio::select! {
            msg = rx.recv() => {
                let Some(message) = msg else {
                    break 'forever;
                };

                waitlist.push(message.on_done);
                times_since_last_action = 0;

                if builder.append(message.data) {
                    writer.write(builder.build()).await;
                    done(&mut waitlist);
                }
            }

            _ = tokio::time::sleep(Duration::from_millis(interval)) => {
                tracing::info!(
                    buffered = builder.size(),
                    times_since_last_action = times_since_last_action,
                    slept_for = interval,
                    "No new messages",
                );
                if builder.size() > 0 {
                    writer.write(builder.build()).await;
                    done(&mut waitlist);
                } else {
                    times_since_last_action += 1;

                    if times_since_last_action >= options.suspend_after {
                        writer.suspend().await;
                    }
                }
            }
        }
    }

    writer.finish().await;

    assert!(rx.is_empty());
    assert!(rx.is_closed());
}

pub fn start_writer(
    format: &Format,
    options: Options,
) -> (Sink, Stats, Box<dyn Future<Output = ()> + '_>) {
    let (tx, rx) = mpsc::channel(options.sink_channel_size);
    let sink = Sink { sender: tx };

    if let Format::Arrow = format {
        use crate::arrow::*;

        let writer = Writer::new(options.spans_per_file);
        let stats = writer.stats().clone();
        let builder = Builder::new(options.builder_flush_threshold, options.builder_capacity);
        let job = run_writer(writer, builder, rx, options);

        return (sink, stats, Box::new(job));
    }

    let Format::Vortex { session, runtime } = format else {
        unreachable!();
    };

    use crate::vortex::*;

    let writer = Writer::new(session, runtime, options.spans_per_file);
    let stats = writer.stats().clone();
    let builder = Builder::new(options.builder_flush_threshold, options.builder_capacity);
    let job = run_writer(writer, builder, rx, options);

    (sink, stats, Box::new(job))
}
