use std::io::{self, Read, Write};
use std::fmt;
use std::convert::TryFrom;
use instruction::parameter::{Direct, Indirect};
use instruction::parameter::{ParamType, ParamTypeOf};
use instruction::parameter::{Register, RegisterError, InvalidRegister};
use instruction::mem_size::MemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

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

impl From<RegisterError> for Error {
    fn from(error: RegisterError) -> Self {
        match error {
            RegisterError::Io(e) => Error::Io(e),
            RegisterError::InvalidRegister(invalid_reg) => Error::InvalidRegister(invalid_reg),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirIndReg {
    Direct(Direct),
    Indirect(Indirect),
    Register(Register),
}

impl GetValue for DirIndReg {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            DirIndReg::Direct(direct) => direct.get_value(vm, context),
            DirIndReg::Indirect(indirect) => indirect.get_value(vm, context),
            DirIndReg::Register(register) => register.get_value(vm, context),
        }
    }

    fn get_value_long(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            DirIndReg::Direct(direct) => direct.get_value_long(vm, context),
            DirIndReg::Indirect(indirect) => indirect.get_value_long(vm, context),
            DirIndReg::Register(register) => register.get_value_long(vm, context),
        }
    }
}

impl MemSize for DirIndReg {
    fn mem_size(&self) -> usize {
        match *self {
            DirIndReg::Direct(direct) => direct.mem_size(),
            DirIndReg::Indirect(indirect) => indirect.mem_size(),
            DirIndReg::Register(register) => register.mem_size(),
        }
    }
}

impl ParamTypeOf for DirIndReg {
    fn param_type(&self) -> ParamType {
        match *self {
            DirIndReg::Direct(_) => ParamType::Direct,
            DirIndReg::Indirect(_) => ParamType::Indirect,
            DirIndReg::Register(_) => ParamType::Register,
        }
    }
}

impl WriteTo for DirIndReg {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match *self {
            DirIndReg::Direct(direct) => direct.write_to(writer),
            DirIndReg::Indirect(indirect) => indirect.write_to(writer),
            DirIndReg::Register(register) => register.write_to(writer),
        }
    }
}

impl<'a, R: Read> TryFrom<(ParamType, &'a mut R)> for DirIndReg {
    type Error = Error;

    fn try_from((param_type, reader): (ParamType, &'a mut R)) -> Result<Self, Self::Error> {
        match param_type {
            ParamType::Direct => Ok(DirIndReg::Direct(Direct::try_from(reader)?)),
            ParamType::Indirect => Ok(DirIndReg::Indirect(Indirect::try_from(reader)?)),
            ParamType::Register => Ok(DirIndReg::Register(Register::try_from(reader)?)),
        }
    }
}

impl fmt::Debug for DirIndReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DirIndReg::Direct(direct) => write!(f, "{:?}", direct),
            DirIndReg::Indirect(indirect) => write!(f, "{:?}", indirect),
            DirIndReg::Register(register) => write!(f, "{:?}", register),
        }
    }
}
