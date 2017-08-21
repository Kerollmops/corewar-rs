use std::io::{Read, Write};
use std::convert::TryFrom;
use instruction::parameter::{Direct, Register, ParamType, InvalidRegister};
use instruction::mem_size::MemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Debug, Clone, Copy)]
pub enum InvalidDirReg {
    InvalidParamType,
    InvalidRegister(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirReg {
    Direct(Direct),
    Register(Register),
}

impl GetValue for DirReg {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            DirReg::Direct(direct) => direct.get_value(vm, context),
            DirReg::Register(register) => register.get_value(vm, context),
        }
    }
}

impl MemSize for DirReg {
    fn mem_size(&self) -> usize {
        match *self {
            DirReg::Direct(direct) => direct.mem_size(),
            DirReg::Register(register) => register.mem_size(),
        }
    }
}

impl WriteTo for DirReg {
    fn write_to<W: Write>(&self, writer: &mut W) {
        match *self {
            DirReg::Direct(direct) => direct.write_to(writer),
            DirReg::Register(register) => register.write_to(writer),
        }
    }
}

impl<'a, R: Read> TryFrom<(ParamType, &'a mut R)> for DirReg {
    type Error = InvalidDirReg;

    fn try_from((param_type, reader): (ParamType, &'a mut R)) -> Result<Self, Self::Error> {
        match param_type {
            ParamType::Direct => Ok(DirReg::Direct(Direct::from(reader))),
            ParamType::Register => match Register::try_from(reader) {
                Ok(reg) => Ok(DirReg::Register(reg)),
                Err(InvalidRegister(reg)) => Err(InvalidDirReg::InvalidRegister(reg)),
            },
            _ => Err(InvalidDirReg::InvalidParamType),
        }
    }
}

impl From<DirReg> for Option<ParamType> {
    fn from(value: DirReg) -> Self {
        match value {
            DirReg::Direct(_) => Some(ParamType::Direct),
            DirReg::Register(_) => Some(ParamType::Register),
        }
    }
}
