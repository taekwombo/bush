// https://www.youtube.com/watch?v=b4mS5UPHh20

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    fn send(&self, data: T) -> () {
        let mut inner = self.shared.inner.lock().unwrap();

        inner.queue.push_back(data);
        drop(inner);

        self.shared.available.notify_one();
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Sender {
            shared: Arc::clone(&self.shared),

        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) -> () {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        if inner.senders == 0 {
            self.shared.available.notify_one()
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> Option<T> {
        let buf_item = self.buffer.pop_front();

        if buf_item.is_some() {
            return buf_item;
        }

        let mut inner = self.shared.inner.lock().unwrap();

        loop {
            match inner.queue.pop_front() {
                Some(v) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Some(v);
                },
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            inner: Mutex::new(Inner {
                queue: Default::default(),
                senders: 1,
            }),
            available: Default::default(),
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared: Arc<Shared<T>> = Arc::new(Default::default());

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            buffer: Default::default()
        },
    )
}

#[cfg(test)]
mod test {
    #[test]
    fn test_channel() {
        use std::thread;

        let (tx, mut rx) = super::channel::<u8>();

        tx.send(10);
        assert_eq!(rx.recv().unwrap(), 10);

        let handle = thread::spawn(move || {
            tx.send(100);
        });

        handle.join().expect("thread didn't panic");

        assert_eq!(rx.recv().unwrap(), 100);
    }

    #[test]
    fn test_channel_clone() {
        struct NotClone;

        let (tx, _) = super::channel::<NotClone>();

        let _ = tx.clone();
    }

    #[test]
    fn test_no_senders() {
        let (_, mut rx) = super::channel::<u8>();

        assert_eq!(None, rx.recv());
    }
}
