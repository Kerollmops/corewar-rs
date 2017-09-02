use std::io::{self, Read};
use std::fmt;
use std::error::Error;
use core::CHAMP_MAX_SIZE;

#[derive(Debug)]
pub enum InvalidProgram {
    TooLong,
}

impl Error for InvalidProgram {
    fn description(&self) -> &str {
        "program size is too long"
    }
}

impl fmt::Display for InvalidProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[derive(Debug)]
pub struct Program {
    inner: Vec<u8>,
}

impl Program {
    pub fn new<R: Read>(program_size: usize, reader: &mut R) -> io::Result<Self> {
        if program_size > CHAMP_MAX_SIZE {
            use self::InvalidProgram::TooLong;
            return Err(io::Error::new(io::ErrorKind::InvalidData, TooLong))
        }

        let mut program = vec![0; program_size];
        reader.read_exact(&mut program)?;

        Ok(Program { inner: program })
    }

    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }
}
