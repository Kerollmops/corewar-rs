use std::io::{self, Read, Write};
use std::fmt;
use instruction::parameter::{Direct, Register, RegisterError, InvalidRegister};
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
pub enum DirReg {
    Direct(Direct),
    Register(Register),
}

impl DirReg {
    pub fn read_from<R: Read>(param_type: ParamType, reader: &mut R) -> Result<Self, Error> {
        match param_type {
            ParamType::Direct => Ok(DirReg::Direct(Direct::read_from(reader)?)),
            ParamType::Register => Ok(DirReg::Register(Register::read_from(reader)?)),
            _ => Err(Error::InvalidParamType(InvalidParamType(param_type))),
        }
    }
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

impl ParamTypeOf for DirReg {
    fn param_type(&self) -> ParamType {
        match *self {
            DirReg::Direct(_) => ParamType::Direct,
            DirReg::Register(_) => ParamType::Register,
        }
    }
}

impl WriteTo for DirReg {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match *self {
            DirReg::Direct(direct) => direct.write_to(writer),
            DirReg::Register(register) => register.write_to(writer),
        }
    }
}

impl fmt::Debug for DirReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DirReg::Direct(direct) => write!(f, "{:?}", direct),
            DirReg::Register(register) => write!(f, "{:?}", register),
        }
    }
}

impl fmt::Display for DirReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DirReg::Direct(dir) => dir.fmt(f),
            DirReg::Register(reg) => reg.fmt(f),
        }
    }
}
