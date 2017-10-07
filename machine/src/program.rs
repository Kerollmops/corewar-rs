use std::io::{self, Read};
use std::fmt;
use std::error::Error;
use core::CHAMP_MAX_SIZE;

#[derive(Debug)]
pub struct InvalidProgramSize {
    size: usize,
}

impl Error for InvalidProgramSize {
    fn description(&self) -> &str {
        "program size is too long"
    }
}

impl fmt::Display for InvalidProgramSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {} exceeds {}", self.description(), self.size, CHAMP_MAX_SIZE)
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    inner: Vec<u8>,
}

impl Program {
    pub fn new<R: Read>(size: usize, reader: &mut R) -> io::Result<Self> {
        if size > CHAMP_MAX_SIZE {
            use self::InvalidProgramSize;
            return Err(io::Error::new(io::ErrorKind::InvalidData, InvalidProgramSize { size }))
        }

        let mut program = vec![0; size];
        reader.read_exact(&mut program)?;

        Ok(Program { inner: program })
    }

    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }
}
