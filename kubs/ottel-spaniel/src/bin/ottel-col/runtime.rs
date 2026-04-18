use std::sync::atomic::{AtomicUsize, Ordering};

use core_affinity::{get_core_ids, set_for_current};
use tokio::runtime::{Builder, Runtime};
#[cfg(not(feature = "free-for-all"))]
use tokio::runtime::{LocalRuntime, LocalOptions};

pub struct RT {
    runtime: Runtime,
    #[cfg(not(feature = "free-for-all"))]
    local_runtime: LocalRuntime,
}

impl RT {
    pub fn new() -> Self {
        let core_ids = get_core_ids().expect("get_core_ids.ok");
        let len = core_ids.len();
        assert!(len > 1);

        #[cfg(not(feature = "free-for-all"))]
        let workers = (len - 1).min(4);

        #[cfg(feature = "free-for-all")]
        let workers = len;

        #[cfg(not(feature = "free-for-all"))]
        let id = core_ids[0];

        #[cfg(not(feature = "free-for-all"))]
        let local = Builder::new_current_thread()
            .on_thread_start(move || {
                set_for_current(id);
            })
            .enable_all()
            .build_local(LocalOptions::default())
            .expect("local-runtime.ok")
            ;

        let idx = AtomicUsize::new(len - 1);

        let runtime = Builder::new_multi_thread()
            .worker_threads(workers)
            .on_thread_start(move || {
                let idx = idx.fetch_sub(1, Ordering::SeqCst);
                let id = core_ids[idx];
                set_for_current(id);
            })
            .enable_all()
            .build()
            .expect("server-runtime.ok")
            ;

        Self {
            #[cfg(not(feature = "free-for-all"))]
            local_runtime: local,
            runtime,
        }
    }

    pub fn run_server_future(&self, fut: impl Future<Output = ()> + Send + 'static) {
        self.runtime.spawn(fut);
    }

    pub fn run_writer_future<'a>(self, fut: Box<dyn Future<Output = ()> + 'a>) {
        let fut = Box::into_pin(fut);

        #[cfg(not(feature = "free-for-all"))]
        self.local_runtime.block_on(fut);

        #[cfg(feature = "free-for-all")]
        self.runtime.block_on(fut);
    }
}
