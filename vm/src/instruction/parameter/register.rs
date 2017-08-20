use std::io::Read;
use std::convert::TryFrom;
use byteorder::ReadBytesExt;
use instruction::parameter::REG_SIZE;
use instruction::mem_size::MemSize;
use instruction::get_value::GetValue;
use virtual_machine::VirtualMachine;
use process::Context;
use core::REG_MAX;

#[derive(Debug, Clone, Copy)]
pub struct InvalidRegister(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(u8);

impl GetValue for Register {
    fn get_value(&self, vm: &VirtualMachine, context: &Context) -> i32 {
        context.registers[*self]
    }
}

impl MemSize for Register {
    fn mem_size(&self) -> usize {
        REG_SIZE
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

impl From<Register> for u8 {
    fn from(reg: Register) -> Self {
        reg.0
    }
}
