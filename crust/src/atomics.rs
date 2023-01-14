//! https://www.youtube.com/watch?v=rMGWeSjctlY
//! https://en.cppreference.com/w/cpp/atomic/memory_order
//! https://en.wikipedia.org/wiki/MESI_protocol

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

/// Holds a luxurious value accessible to a one thread at a time.
pub struct Lux<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Lux<T> where T: Send {}

impl<T> Lux<T> {
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            // Ordering::Acquire
            // A load operation with this memory order performs the acquire operation on the
            // affected memory location: no reads or writes in the current thread can be reordered
            // before this load. All writes in other threads that release the same atomic variable
            // are visible in the current thread (see Release-Acquire ordering below).
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // When failed to take the value, spin until locked changes value to false.
            while self.locked.load(Ordering::Relaxed) {
                std::hint::spin_loop();
            }
        }

        let r = f(unsafe { &mut *self.value.get() });

        // Ordering::Release
        // A store operation with this memory order performs the release operation: no reads or writes
        // in the current thread can be reordered after this store.
        // All writes in the current thread are visible in other threads that acquire the same atomic
        // variable and writes that carry a dependency into the atomic variable become visible in other
        // threads that consume the same atomic.
        self.locked.store(false, Ordering::Release);

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::{spawn, JoinHandle};

    #[test]
    fn lock() {
        let value: &'static _ = Box::leak(Box::new(Lux::new(0)));
        let threads = (0..10)
            .map(|_| {
                spawn(|| -> () {
                    for _ in 0..1000 {
                        value.with_lock(|v| *v += 1);
                    }
                })
            })
            .collect::<Vec<JoinHandle<_>>>();

        for t in threads {
            t.join().unwrap();
        }

        let v = value.with_lock(|value| *value);
        assert_eq!(v, 10 * 1000);
    }

    #[test]
    fn seq_cst() {
        use std::sync::atomic::AtomicUsize;

        let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

        spawn(|| {
            x.store(true, Ordering::Release);
        });

        spawn(|| {
            y.store(true, Ordering::Release);
        });

        let t1 = spawn(|| {
            // Must see value true in x variable before going forward.
            while !x.load(Ordering::Acquire) {}

            // Check value of y after store operations with Ordering::Release.
            // If no such operations happened yet then it will see false.
            if y.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });

        let t2 = spawn(|| {
            // Must see value true in y variable before going forward.
            while !y.load(Ordering::Acquire) {}

            // Check value of x after store operations with Ordering::Release.
            // If no such operations happened yet then it will see false.
            if x.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();

        let z = z.load(Ordering::SeqCst);

        // Possible values for z:
        // 2 - x stored, y stored, then t1 and t2 got to the if statements.
        // 1 - x stored, t1 got to the if statement and didn't increment.
        // 0 - x stored, y stored, t1 and t2 observe false in if statement
        // (they observe x and y stores in different order).
        println!("{}", z);
    }
}
