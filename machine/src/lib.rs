#![feature(try_from)]
#![feature(concat_idents)]
#![feature(const_fn)]

extern crate byteorder;
#[macro_use] extern crate log;
extern crate core;

mod machine;
pub mod champion;
pub mod program;
pub mod arena;
mod process;
pub mod instruction;

pub use machine::{Machine, CycleExecute};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
