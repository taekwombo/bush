pub mod builder;
pub mod convert;
pub mod schema;

use builder::SpanBatch;

use std::time::Duration;
use tokio::time::sleep;
use tokio::task::{spawn, JoinHandle};
use tokio::sync::{mpsc, oneshot};

pub type Sender = mpsc::Sender<(oneshot::Sender<()>, SpanBatch)>;

// Should use sync::Notify
struct Confirm {
    senders: Vec<oneshot::Sender<()>>,
}

impl Confirm {
    fn new(capacity: usize) -> Self {
        Self { senders: Vec::with_capacity(capacity) }
    }

    fn push(&mut self, sender: oneshot::Sender<()>) -> &mut Self {
        self.senders.push(sender);
        self
    }

    fn confirm(&mut self) {
        for s in self.senders.drain(..) {
            s.send(()).expect("sent");
        }

        assert!(self.senders.len() == 0);
    }
}

pub fn create_writer(channel_size: usize) -> (Sender, JoinHandle<()>) {
    let (tx, mut rx) = mpsc::channel::<(oneshot::Sender<()>, SpanBatch)>(channel_size);

    // rx should be converted into batched stream
    // message should come with one-shot channel
    // add file writer for batches
    let handle = spawn(async move {
        let mut builder = builder::BatchWriter::new(128, 256);
        let mut cf = Confirm::new(128);

        'forever: loop {
            tokio::select! {
                message = rx.recv() => {
                    match message {
                        None => {
                            break 'forever;
                        }
                        Some((complete, b)) => {
                            cf.push(complete);

                            // println!("Appending batch of {}", b.len());
                            println!("-{}", rx.len());
                            if builder.append(&b) {
                                let batch = builder.build();
                                println!("Built on demand: {:?}", batch.map(|b| b.num_rows()));
                                cf.confirm();
                            }
                        }
                    }
                }
                _ = sleep(Duration::from_millis(500)) => {
                    let batch = builder.build();
                    cf.confirm();
                    println!("[{}] should sync now: {}/{} => {:?}", rx.len(), builder.written, builder.threshold, batch.map(|b| b.num_rows()));
                }
            }
        }
    });

    (tx, handle)
}
