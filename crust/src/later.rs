//! https://www.youtube.com/watch?v=ThjvMReOXYM

#[allow(dead_code)]
fn generators(start: u8, repeat: u8, inc_by: u8) -> (u8, u8) {
    //! https://doc.rust-lang.org/std/ops/trait.Generator.html
    //! https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html
    //!
    //! Building block of async/await syntax.

    use std::ops::{Generator, GeneratorState};
    use std::pin::Pin;

    // Generator<R = ()> R - type accepted by resume function.
    //  type Yield - type yielded by generator.
    //  type Return - type returned by generator.
    //
    // fn pin(self: Pin<&mut Self>, arg: R) -> GeneratorState<Yield, Return>

    fn resume_generator<G>(generator: &mut G, resume_value: u8) -> Option<u8>
    where
        G: Generator<u8, Yield = u8, Return = u8> + Unpin,
    {
        match Pin::new(generator).resume(resume_value) {
            GeneratorState::Yielded(y) => Some(y),
            _ => None,
        }
    }

    // Start counting from init, adding R for each resume call.
    let mut count_from = |init: u8 /* first resume arg here */| -> u8 {
        let mut count = init;

        loop {
            let add_next = yield count;

            // If caller doesn't want to increment any further - stop.
            if add_next == 0 {
                break;
            }

            count += add_next;
        }

        init
    };

    let mut result = 0;

    // Resume generator first time providing start argument.
    if let Some(f) = resume_generator(&mut count_from, start) {
        result = f;
    }

    // For each repeat add inc_by to counter.
    for _ in 0..repeat {
        if let Some(f) = resume_generator(&mut count_from, inc_by) {
            result = f;
        }
    }

    match Pin::new(&mut count_from).resume(0) {
        GeneratorState::Complete(complete) => (result, complete),
        _ => unreachable!(),
    }
    // This example could return struct that implements Iterator - it would be easier to observe
    // changes.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_generator() {
        let init = 0;
        let repeats = 3;

        {
            let (last_yield, return_value) = generators(init, repeats, 1);
            assert_eq!(3, last_yield);
            assert_eq!(init, return_value);
        }
        {
            let (last_yield, return_value) = generators(init, repeats, 2);
            assert_eq!(6, last_yield);
            assert_eq!(init, return_value);
        }
    }

    #[test]
    fn pin() {
        //! https://cfsamson.github.io/books-futures-explained/5_pin.html

        use std::marker::PhantomPinned;
        use std::pin::Pin;

        /// Self referential struct.
        struct Text {
            a: String,
            b: *const String,
        }

        impl Text {
            fn new(txt: &str) -> Self {
                Self {
                    a: String::from(txt),
                    b: std::ptr::null(),
                }
            }

            fn init(&mut self) {
                let self_ref: *const String = &self.a;
                self.b = self_ref;
            }

            fn a(&self) -> &str {
                &self.a
            }

            fn b(&self) -> &String {
                unsafe { &*(self.b) }
            }
        }

        {
            let mut x = Text::new("orange");
            let mut y = Text::new("apple");
            x.init();
            y.init();

            assert_eq!(x.a(), "orange");
            assert_eq!(x.b(), "orange");
            assert_eq!(y.a(), "apple");
            assert_eq!(y.b(), "apple");

            // Raw pointers still point to the old data.
            std::mem::swap(&mut x, &mut y);

            assert_eq!(x.a(), "apple");
            assert_eq!(x.b(), "orange");
            assert_eq!(y.a(), "orange");
            assert_eq!(y.b(), "apple");
        }

        struct PinnedText {
            a: String,
            b: *const String,
            _marker: PhantomPinned,
        }

        impl PinnedText {
            fn new(txt: &str) -> Self {
                Self {
                    a: String::from(txt),
                    b: std::ptr::null(),
                    _marker: PhantomPinned, // This makes our type `!Unpin`
                }
            }
            fn init<'a>(self: Pin<&'a mut Self>) {
                let self_ptr: *const String = &self.a;
                let this = unsafe { self.get_unchecked_mut() };
                this.b = self_ptr;
            }

            fn a<'a>(self: Pin<&'a Self>) -> &'a str {
                &self.get_ref().a
            }

            fn b<'a>(self: Pin<&'a Self>) -> &'a String {
                unsafe { &*(self.b) }
            }
        }

        {
            let mut x = PinnedText::new("orange");
            let mut y = PinnedText::new("apple");
            let mut x = unsafe { Pin::new_unchecked(&mut x) };
            let mut y = unsafe { Pin::new_unchecked(&mut y) };

            PinnedText::init(x.as_mut());
            PinnedText::init(y.as_mut());

            assert_eq!(PinnedText::a(x.as_ref()), "orange");
            assert_eq!(PinnedText::b(x.as_ref()), "orange");
            assert_eq!(PinnedText::a(y.as_ref()), "apple");
            assert_eq!(PinnedText::b(y.as_ref()), "apple");

            // Does not compile.
            // error[E0277]: `PhantomPinned` cannot be unpinned
            // std::mem::swap(x.get_mut(), y.get_mut());
        }
    }

    #[test]
    #[cfg(feature = "future")]
    fn future_impl() {
        use smol::block_on;
        use std::future::Future;
        use std::ops::DerefMut;
        use std::pin::Pin;
        use std::sync::{Arc, Mutex};
        use std::task::{Context, Poll};
        use std::thread;

        struct ReadFile<T: AsRef<str>> {
            path: T,
            work: Arc<Mutex<Option<()>>>,
            data: Arc<Mutex<Vec<u8>>>,
        }

        impl<T: AsRef<str>> ReadFile<T> {
            fn new(path: T) -> Self {
                Self {
                    path,
                    work: Default::default(),
                    data: Default::default(),
                }
            }
        }

        impl<T: AsRef<str>> Future for ReadFile<T> {
            type Output = Vec<u8>;

            fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
                let mut working = self.work.lock().unwrap();

                match *working {
                    None => {
                        working.deref_mut().replace(());

                        // This is just an example. Spawning thread for each future is ok.
                        let path = self.path.as_ref().to_string();
                        let data = self.data.clone();
                        let waker = context.waker().clone();
                        thread::spawn(move || {
                            let res = std::fs::read(path).unwrap();
                            let mut data = data.lock().unwrap();
                            *data.deref_mut() = res;
                            waker.wake();
                        });

                        Poll::Pending
                    }
                    Some(_) => {
                        let mut data = self.data.lock().unwrap();

                        if data.len() == 0 {
                            Poll::Pending
                        } else {
                            let mut vec = Vec::new();
                            std::mem::swap(data.deref_mut(), &mut vec);

                            Poll::Ready(vec)
                        }
                    }
                }
            }
        }

        block_on(async {
            let read = ReadFile::new("Cargo.toml");
            let result = read.await;
            let content = String::from_utf8_lossy(&result);
            println!("Cargo.toml reads:\n\n{}", content);
        });
    }
}
