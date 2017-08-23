use std::path::Path;
use std::io;
use program::{Program, InvalidProgram};

// FIXME: impl Error trait
#[derive(Debug)]
pub enum InvalidChampion {
    IoError(io::Error),
    ProgramError(InvalidProgram),
}

#[derive(Debug)]
pub struct Champion {
    pub id: i32,
    pub name: String,
    pub comment: String,
    pub program: Program,
    _private: (),
}

impl Champion {
    pub fn new<P: AsRef<Path>>(id: i32, path: P) -> Result<Self, InvalidChampion> {
        unimplemented!()
    }
}
