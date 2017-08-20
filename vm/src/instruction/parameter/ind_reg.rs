use std::io::Read;
use std::convert::TryFrom;
use instruction::parameter::{Indirect, Register, ParamType, InvalidRegister};
use instruction::mem_size::MemSize;
use instruction::get_value::GetValue;
use virtual_machine::VirtualMachine;
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
    fn get_value(&self, vm: &VirtualMachine, context: &Context) -> i32 {
        match *self {
            IndReg::Indirect(indirect) => indirect.get_value(vm, context),
            IndReg::Register(register) => register.get_value(vm, context),
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
