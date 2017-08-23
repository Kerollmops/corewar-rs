use core::PROG_MAX_SIZE;

// FIXME: impl Error trait
#[derive(Debug)]
pub enum InvalidProgram {
    TooLong,
}

#[derive(Debug)]
pub struct Program {
    inner: Vec<u8>,
}

impl Program {
    pub fn from_slice(slice: &[u8]) -> Result<Self, InvalidProgram> {
        if slice.len() > PROG_MAX_SIZE {
            return Err(InvalidProgram::TooLong)
        }
        Ok(Program { inner: slice.to_owned() })
    }

    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }
}
