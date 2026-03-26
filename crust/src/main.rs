#![feature(dropck_eyepatch, stmt_expr_attributes, coroutines, coroutine_trait)]

pub mod atomics;
pub mod cell;
pub mod channel;
pub mod drop_check;
pub mod fattos;
pub mod funs;
pub mod iter;
pub mod later;
#[cfg(target_arch = "x86_64")]
pub mod lego;
#[cfg(feature = "bindgen")]
pub mod soda;
pub mod strtok;

fn main() {
    println!("Hello, world!");
}
