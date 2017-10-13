#![feature(const_size_of)]

extern crate byteorder;
#[macro_use] extern crate log;
pub extern crate core;

mod machine;
pub mod champion;
pub mod program;
pub mod arena;
mod process;
pub mod instruction;

pub use machine::{Machine, CycleExecute};
