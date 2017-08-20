#![feature(try_from)]

extern crate byteorder;
extern crate core;

mod virtual_machine;
mod player;
mod arena;
mod process;
mod instruction;

pub use virtual_machine::VirtualMachine;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
