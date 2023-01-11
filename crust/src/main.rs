#![feature(dropck_eyepatch)]

pub mod atomics;
pub mod cell;
pub mod channel;
pub mod drop_check;
pub mod iter;
pub mod strtok;

// Plan:
// crust of rust until Future impls are here
// dyn dispatch - vtable
// asm!
// ffi

fn main() {
    println!("Hello, world!");
}
