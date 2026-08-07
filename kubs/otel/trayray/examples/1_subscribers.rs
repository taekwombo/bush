//! ## Overview (simple)
//! Tracing works with two components: publishers (e.g. [info!]) and subscribers (e.g. [tracing::subscriber]).
//! 
//! Tracing is modeled around two data pieces:
//! - [Span] which represents and operation
//! - [Event] which represents single point in time within an operation
//!
//! Other components:
//! - [tracing::dispatcher] - component responsible for sending trace events to subscribers
//!   When you create new span it will access the default dispatcher and ask Subscriber for new span [Id].
//!
//!   ```
//!   /*tracing::Span*/
//!   fn new(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span {
//!     dispatcher::get_default(|dispatch| Self::new_with(meta, values, dispatch))
//!   } 
//!   ```
//!   
//! - [tracing::subscriber] - component responsible for recording trace data / reacting to trace data
//!
//! [Id]: tracing::span::Id
//! [Span]: tracing::Span
//! [Event]: tracing::Event
//! [info!]: tracing::info!

use tracing::dispatcher::Dispatch;

fn main() {
    // Subscriber that logs tracing data.
    let fmt = || tracing_subscriber::fmt().compact().finish();

    // Sets default subscriber for the duration of the closure.
    tracing::dispatcher::with_default(&Dispatch::new(fmt()), || {
        tracing::info!("Scoped default dispatcher works.");
    });

    // This log line will not produce anything in the console.
    // Default subscriber is the Noop.
    tracing::info!("No default or global dispatcher yet set.");

    {
        // This sets the default dispatcher for the duration of the lifetime of the guard.
        let default_guard = tracing::dispatcher::set_default(&Dispatch::new(fmt()));

        tracing::info!("Default guard alive, log works!");
    }

    // When the default_guard drops, default subscriber is brought back to Noop.
    tracing::info!("No default dispatcher set anymore.");

    // So let's try to set the default dispatcher once and for all!
    let default_guard = tracing::dispatcher::set_default(&Dispatch::new(fmt()));
    Box::leak(Box::new(default_guard));

    tracing::info!("Now, default dispatcher will be set until the end of the program!");
    tracing::info!("But, does it work for other threads? No.");

    std::thread::spawn(|| {
        tracing::info!("Another thread says hello!");
    }).join().unwrap();

    tracing::info!("In order to set default dispatcher for all thread, set_global_default is used.");

    // Set global default so that there is formatter for all threads.
    tracing::dispatcher::set_global_default(Dispatch::new(fmt())).unwrap();

    std::thread::spawn(|| {
        tracing::info!("Now, another thread can say hello!");
    }).join().unwrap();
}
