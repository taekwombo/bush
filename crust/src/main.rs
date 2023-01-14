#![feature(dropck_eyepatch, generators, generator_trait, never_type)]

pub mod atomics;
pub mod cell;
pub mod channel;
pub mod drop_check;
pub mod fattos;
pub mod funs;
pub mod iter;
pub mod later;
pub mod strtok;

// Plan:
// crust of rust until Future impls are here
// Serde refresh - https://www.youtube.com/watch?v=BI_bHCGRgMY
// dyn dispatch - vtable
// asm!
// ffi

fn main() {
    println!("Hello, world!");
}
