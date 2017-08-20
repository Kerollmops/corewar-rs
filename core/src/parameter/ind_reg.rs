use std::io::Read;
use std::convert::TryFrom;
use parameter::{Indirect, Register, ParamType, InvalidRegister};
use mem_size::MemSize;

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
