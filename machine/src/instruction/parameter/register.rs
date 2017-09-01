use std::io::{Read, Write};
use std::convert::TryFrom;
use std::fmt;
use byteorder::{ReadBytesExt, WriteBytesExt};
use instruction::parameter::REG_SIZE;
use instruction::mem_size::MemSize;
use instruction::const_mem_size::ConstMemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;
use core::REG_MAX;

#[derive(Debug, Clone, Copy)]
pub struct InvalidRegister(pub u8);

impl fmt::Display for InvalidRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "invalid register number")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(u8);

impl ConstMemSize for Register {
    const MEM_SIZE: usize = REG_SIZE;
}

impl GetValue for Register {
    fn get_value(&self, _vm: &Machine, context: &Context) -> i32 {
        context.registers[*self]
    }
}

impl MemSize for Register {
    fn mem_size(&self) -> usize {
        Register::MEM_SIZE
    }
}

impl WriteTo for Register {
    fn write_to<W: Write>(&self, writer: &mut W) {
        let _ = writer.write_u8(self.0);
    }
}

impl<'a, R: Read> TryFrom<&'a mut R> for Register {
    type Error = InvalidRegister;

    fn try_from(reader: &'a mut R) -> Result<Self, InvalidRegister> {
        match reader.read_u8().unwrap() {
            value @ 1...REG_MAX => Ok(Register(value)),
            value => Err(InvalidRegister(value)),
        }
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
