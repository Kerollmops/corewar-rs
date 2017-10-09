use std::io::{self, Read, Write};
use std::fmt;
use std::convert::TryFrom;
use byteorder::{ReadBytesExt, WriteBytesExt};
use instruction::parameter::REG_SIZE;
use instruction::mem_size::ConstMemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;
pub use core::REG_MAX;

#[derive(Debug)]
pub struct InvalidRegister(u8);

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidRegister(InvalidRegister),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<InvalidRegister> for Error {
    fn from(error: InvalidRegister) -> Self {
        Error::InvalidRegister(error)
    }
}

impl fmt::Display for InvalidRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "invalid register number")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(u8);

impl Register {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl ConstMemSize for Register {
    const MEM_SIZE: usize = REG_SIZE;
}

impl GetValue for Register {
    fn get_value(&self, _vm: &Machine, context: &Context) -> i32 {
        context.registers[*self]
    }
}

impl WriteTo for Register {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u8(self.0)
    }
}

impl<'a, R: Read> TryFrom<&'a mut R> for Register {
    type Error = Error;

    fn try_from(reader: &'a mut R) -> Result<Self, Error> {
        let value = reader.read_u8()?;
        Ok(Register::try_from(value)?)
    }
}

impl TryFrom<u8> for Register {
    type Error = InvalidRegister;

    fn try_from(value: u8) -> Result<Self, InvalidRegister> {
        match value {
            value @ 1...REG_MAX => Ok(Register(value)),
            value => Err(InvalidRegister(value)),
        }
    }
}

impl From<Register> for u8 {
    fn from(reg: Register) -> Self {
        reg.0
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}
