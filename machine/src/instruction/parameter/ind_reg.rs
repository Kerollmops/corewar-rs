use std::io::{self, Read, Write};
use std::fmt;
use instruction::parameter::{Indirect, Register, RegisterError, InvalidRegister};
use instruction::parameter::{ParamType, ParamTypeOf};
use instruction::parameter::InvalidParamType;
use instruction::mem_size::MemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidParamType(InvalidParamType),
    InvalidRegister(InvalidRegister)
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

impl From<RegisterError> for Error {
    fn from(error: RegisterError) -> Self {
        match error {
            RegisterError::Io(e) => Error::Io(e),
            RegisterError::InvalidRegister(invalid_reg) => Error::InvalidRegister(invalid_reg),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndReg {
    Indirect(Indirect),
    Register(Register),
}

impl IndReg {
    pub fn read_from<R: Read>(param_type: ParamType, reader: & mut R) -> Result<Self, Error> {
        match param_type {
            ParamType::Indirect => Ok(IndReg::Indirect(Indirect::read_from(reader)?)),
            ParamType::Register => Ok(IndReg::Register(Register::read_from(reader)?)),
            _ => Err(Error::InvalidParamType(InvalidParamType(param_type))),
        }
    }
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

impl ParamTypeOf for IndReg {
    fn param_type(&self) -> ParamType {
        match *self {
            IndReg::Indirect(_) => ParamType::Indirect,
            IndReg::Register(_) => ParamType::Register,
        }
    }
}

impl WriteTo for IndReg {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match *self {
            IndReg::Indirect(indirect) => indirect.write_to(writer),
            IndReg::Register(register) => register.write_to(writer),
        }
    }
}

impl fmt::Debug for IndReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IndReg::Indirect(indirect) => write!(f, "{:?}", indirect),
            IndReg::Register(register) => write!(f, "{:?}", register),
        }
    }
}

impl fmt::Display for IndReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IndReg::Indirect(ind) => ind.fmt(f),
            IndReg::Register(reg) => reg.fmt(f),
        }
    }
}
