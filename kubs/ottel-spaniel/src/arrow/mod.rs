use std::time::Duration;

use tokio::time::sleep;
use tokio::sync::{mpsc, oneshot};

use crate::schema::SpanData;

mod build;
mod write;

pub use write::Writer;

pub async fn sink(writer: Writer, mut rx: mpsc::Receiver<crate::Message>) -> mpsc::Receiver<crate::Message> {
    struct BobbySinker {
        waitlist: Vec<oneshot::Sender::<()>>,
        writer: Writer,
    }

    impl BobbySinker {
        fn append(&mut self, waiting: oneshot::Sender::<()>, data: &[SpanData]) {
            self.waitlist.push(waiting);

            if self.writer.append(&data) {
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
            println!("sink.save {} {:?}", self.writer.builder.size, std::thread::current().id());
            self.writer.save();
            self.confirm();
        }

        fn end(mut self) {
            self.save();
            self.writer.finish();

            assert!(self.waitlist.is_empty());
        }
    }

    let mut sinker = BobbySinker {
        waitlist: Vec::with_capacity(128),
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
