use tokio::runtime::{Builder, Runtime};

pub struct RT {
    runtime: Runtime,
    #[cfg(not(feature = "free-for-all"))]
    local_runtime: tokio::runtime::LocalRuntime,
}

impl RT {
    #[cfg(feature = "free-for-all")]
    pub fn new() -> Self {
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("server-runtime.ok");

        Self { runtime }
    }

    #[cfg(not(feature = "free-for-all"))]
    pub fn new() -> Self {
        use core_affinity::{get_core_ids, set_for_current};
        use std::sync::atomic::{AtomicUsize, Ordering};

        let core_ids = get_core_ids().expect("get_core_ids.ok");
        let len = core_ids.len();
        assert!(len > 1);

        let workers = (len - 1).min(4);
        let id = core_ids[0];

        let local = Builder::new_current_thread()
            .on_thread_start(move || {
                set_for_current(id);
            })
            .enable_all()
            .build_local(tokio::runtime::LocalOptions::default())
            .expect("local-runtime.ok");

        let idx = AtomicUsize::new(len - 1);

        let runtime = Builder::new_multi_thread()
            .worker_threads(workers)
            .on_thread_start(move || {
                let idx = idx.fetch_sub(1, Ordering::SeqCst) % len;
                let id = core_ids[idx];
                set_for_current(id);
            })
            .enable_all()
            .build()
            .expect("server-runtime.ok");

        Self {
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
