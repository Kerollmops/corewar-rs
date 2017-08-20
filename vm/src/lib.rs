#![feature(try_from)]

extern crate byteorder;
extern crate core;

mod machine;
mod player;
mod arena;
mod process;
mod instruction;

pub use machine::Machine;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
