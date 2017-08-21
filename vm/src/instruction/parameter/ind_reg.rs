use std::io::{Read, Write};
use std::convert::TryFrom;
use instruction::parameter::{Indirect, Register, ParamType, InvalidRegister};
use instruction::mem_size::MemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Debug, Clone, Copy)]
pub enum InvalidIndReg {
    InvalidParamType,
    InvalidRegister(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndReg {
    Indirect(Indirect),
    Register(Register),
}

impl GetValue for IndReg {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            IndReg::Indirect(indirect) => indirect.get_value(vm, context),
            IndReg::Register(register) => register.get_value(vm, context),
        }
    }

    fn get_value_long(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            IndReg::Indirect(indirect) => indirect.get_value_long(vm, context),
            IndReg::Register(register) => register.get_value_long(vm, context),
        }
    }
}

impl MemSize for IndReg {
    fn mem_size(&self) -> usize {
        match *self {
            IndReg::Indirect(indirect) => indirect.mem_size(),
            IndReg::Register(register) => register.mem_size(),
        }
    }
}

impl WriteTo for IndReg {
    fn write_to<W: Write>(&self, writer: &mut W) {
        match *self {
            IndReg::Indirect(indirect) => indirect.write_to(writer),
            IndReg::Register(register) => register.write_to(writer),
        }
    }
}

impl<'a, R: Read> TryFrom<(ParamType, &'a mut R)> for IndReg {
    type Error = InvalidIndReg;

    fn try_from((param_type, reader): (ParamType, &'a mut R)) -> Result<Self, Self::Error> {
        match param_type {
            ParamType::Indirect => Ok(IndReg::Indirect(Indirect::from(reader))),
            ParamType::Register => match Register::try_from(reader) {
                Ok(reg) => Ok(IndReg::Register(reg)),
                Err(InvalidRegister(reg)) => Err(InvalidIndReg::InvalidRegister(reg)),
            },
            _ => Err(InvalidIndReg::InvalidParamType),
        }
    }
}

impl From<IndReg> for Option<ParamType> {
    fn from(value: IndReg) -> Self {
        match value {
            IndReg::Indirect(_) => Some(ParamType::Indirect),
            IndReg::Register(_) => Some(ParamType::Register),
        }
    }
}
