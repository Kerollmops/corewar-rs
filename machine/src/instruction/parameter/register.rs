use std::io::{self, Read, Write};
use std::{ops, fmt};
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
    pub fn new(number: u8) -> Result<Self, InvalidRegister> {
        match number {
            number @ 1...REG_MAX => Ok(Register(number)),
            number => Err(InvalidRegister(number)),
        }
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let value = reader.read_u8()?;
        let register = Register::new(value)?;
        Ok(register)
    }
}

impl ops::Deref for Register {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
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

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}
