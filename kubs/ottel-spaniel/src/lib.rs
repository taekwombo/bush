// 1. Create simple SpanData write and read with filters.
// 2. Extend simple SpanData with primitive .resource attributes.
// 3. Read traces via API:
//    Filter by: trace_id, span_id, parent_span_id
//             : span_name, span primitive attributes
//             : only `AND` filtering

pub mod convert;
pub mod schema;
pub mod arrow;
pub mod vortex;
mod misc;

use tokio::sync::{mpsc, oneshot};

pub type Message = (oneshot::Sender<()>, Vec<schema::SpanData>);

pub struct BobbySinker {
    batch_sink: mpsc::Sender<Message>,
}

impl BobbySinker {
    fn new(batch_sink: mpsc::Sender<Message>) -> Self {
        Self { batch_sink }
    }

    pub async fn write(&self, batch: Vec<schema::SpanData>) -> () {
        let (tx, rx) = oneshot::channel::<()>();

        self.batch_sink.send((tx, batch)).await.expect("sink.batch_sink.alive");
        rx.await.expect("sink.oneshot.ok");
    }
}

pub fn start<T, S>(
    channel_size: usize,
    run_server: impl FnOnce(BobbySinker) -> T,
    run_sink: impl FnOnce(mpsc::Receiver<Message>) -> S,
)
where
    T: Future<Output = ()>,
    T: Send + 'static,
    S: Future<Output = mpsc::Receiver<Message>>,
{
    let (tx, rx) = mpsc::channel::<Message>(channel_size);
    let sinker = BobbySinker::new(tx);

    #[cfg(not(feature = "free-for-all"))]
    let mut sink_rt = tokio::runtime::Builder::new_current_thread();
    let mut server_rt = tokio::runtime::Builder::new_multi_thread();

    #[cfg(not(feature = "free-for-all"))]
    {
        use std::sync::{Arc, Mutex};

        let mut core_ids = core_affinity::get_core_ids().expect("get_core_ids.ok");
        assert!(!core_ids.is_empty());

        let sink_id = core_ids.pop().unwrap();
        let workers = core_ids.len().min(4);
        let core_ids = Arc::new(Mutex::new(core_ids));

        sink_rt
            .on_thread_start(move || {
                core_affinity::set_for_current(sink_id);
            });

        server_rt
            .worker_threads(workers)
            .on_thread_start(move || {
                let id = core_ids.clone().lock().expect("mutex.ok").pop().expect("pop.ok");
                core_affinity::set_for_current(id);
            });
    }

    #[cfg(not(feature = "free-for-all"))]
    let sink_rt = sink_rt
        .enable_all()
        .build_local(tokio::runtime::LocalOptions::default())
        .expect("sink_rt.ok");

    let server_rt = server_rt
        .enable_all()
        .build()
        .expect("server_rt.ok");

    server_rt.spawn(run_server(sinker));

    #[cfg(not(feature = "free-for-all"))]
    sink_rt.block_on(async move {
        let rx = run_sink(rx).await;

        assert!(rx.is_empty());
        assert!(rx.is_closed());
    });

    // Use .spawn(...) to allow sink to be run on other workers.
    #[cfg(feature = "free-for-all")]
    server_rt.block_on(async move {
        let rx = run_sink(rx).await;

        assert!(rx.is_empty());
        assert!(rx.is_closed());
    });
}
