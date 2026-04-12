use std::time::Duration;

use tokio::time::sleep;
use tokio::sync::{mpsc, oneshot};

use crate::schema::SpanData;

mod build;
mod read;
mod write;

pub use read::Filter;
pub use write::Writer;

pub async fn sink(
    builder_size: usize,
    writer: Writer,
    mut rx: mpsc::Receiver<crate::Message>,
) -> mpsc::Receiver<crate::Message> {
    struct Sink {
        waitlist: Vec<oneshot::Sender::<()>>,
        builder: build::Builder,
        writer: Writer,
    }

    impl Sink {
        fn append(&mut self, waiting: oneshot::Sender::<()>, data: &[SpanData]) {
            self.waitlist.push(waiting);

            if self.builder.append(&data) {
                self.save();
            }
        }

        fn confirm(&mut self) {
            for i in self.waitlist.drain(..) {
                i.send(()).expect("oneshot.send.ok");
            }

            assert!(self.waitlist.is_empty());
        }

        fn save(&mut self) {
            if self.builder.size == 0 {
                return;
            }

            tracing::info!(
                builder.size = self.builder.size, 
                thread_id = ?std::thread::current().id(),
                "sink.save",
            );
            self.writer.save(self.builder.build());
            self.confirm();
        }

        fn end(mut self) {
            self.save();
            self.writer.finish();

            assert!(self.waitlist.is_empty());
            assert!(self.builder.size == 0);
        }
    }

    let mut sinker = Sink {
        waitlist: Vec::with_capacity(128),
        builder: build::Builder::new(builder_size, builder_size * 2),
        writer,
    };

    'forever: loop {
        tokio::select! {
            message = rx.recv() => {
                let Some((waiting, batch)) = message else {
                    break 'forever;
                };

                sinker.append(waiting, batch.as_slice());
            },
            
            _ = sleep(Duration::from_millis(1000)) => {
                sinker.save();
            },
        }
    }

    sinker.end();

    rx
}
