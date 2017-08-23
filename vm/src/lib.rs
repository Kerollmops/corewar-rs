#![feature(try_from)]
#![feature(concat_idents)]

extern crate byteorder;
extern crate core;

mod machine;
mod champion;
mod arena;
mod process;
mod program;
mod instruction;

pub use machine::Machine;

// let mut file = File::open(&champion.program_path)?;
// if file.metadata()?.len() > CHAMP_MAX_SIZE as u64 {
//     return Err(io::Error::new(io::ErrorKind::Other, "champion size is too big"))
// }
// let mut content = Vec::new();
// file.read_to_end(&mut content)?;

// let mut cursor = io::Cursor::new(content);

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
