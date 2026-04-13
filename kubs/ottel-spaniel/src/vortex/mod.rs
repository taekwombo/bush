use std::time::Duration;

use tokio::time::sleep;
use tokio::sync::{mpsc, oneshot};

use crate::schema::SpanData;

mod build;
mod read;
mod write;

pub use write::Writer;

pub async fn sink<'a>(
    builder_size: usize,
    writer: Writer<'a>,
    mut rx: mpsc::Receiver<crate::Message>,
) -> mpsc::Receiver<crate::Message> {
    struct Sink<'a> {
        waitlist: Vec<oneshot::Sender::<()>>,
        builder: build::Builder,
        writer: Writer<'a>,
    }

    impl Sink<'_> {
        fn append(&mut self, waiting: oneshot::Sender::<()>, data: Vec<SpanData>) {
            self.waitlist.push(waiting);

            if self.builder.append(data) {
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

                sinker.append(waiting, batch);
            },
            
            _ = sleep(Duration::from_millis(1000)) => {
                sinker.save();
            },
        }
    }

    sinker.save();
    sinker.writer.finish();

    assert!(sinker.waitlist.is_empty());
    assert!(sinker.builder.size == 0);

    rx
}
