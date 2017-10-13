use std::io::{self, Read, Write};
use std::fmt;
use instruction::parameter::{AltDirect, Register};
use instruction::parameter::{ParamType, ParamTypeOf};
use instruction::parameter::InvalidParamType;
use instruction::parameter::dir_reg::Error;
use instruction::mem_size::MemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AltDirReg {
    AltDirect(AltDirect),
    Register(Register),
}

impl AltDirReg {
    pub fn read_from<R: Read>(param_type: ParamType, reader: &mut R) -> Result<Self, Error> {
        match param_type {
            ParamType::Direct => Ok(AltDirReg::AltDirect(AltDirect::read_from(reader)?)),
            ParamType::Register => Ok(AltDirReg::Register(Register::read_from(reader)?)),
            _ => Err(Error::InvalidParamType(InvalidParamType(param_type))),
        }
    }
}

impl GetValue for AltDirReg {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            AltDirReg::AltDirect(alt_direct) => alt_direct.get_value(vm, context),
            AltDirReg::Register(register) => register.get_value(vm, context),
        }
    }
}

impl MemSize for AltDirReg {
    fn mem_size(&self) -> usize {
        match *self {
            AltDirReg::AltDirect(alt_direct) => alt_direct.mem_size(),
            AltDirReg::Register(register) => register.mem_size(),
        }
    }
}

impl ParamTypeOf for AltDirReg {
    fn param_type(&self) -> ParamType {
        match *self {
            AltDirReg::AltDirect(_) => ParamType::Direct,
            AltDirReg::Register(_) => ParamType::Register,
        }
    }
}

impl WriteTo for AltDirReg {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match *self {
            AltDirReg::AltDirect(alt_direct) => alt_direct.write_to(writer),
            AltDirReg::Register(register) => register.write_to(writer),
        }
    }
}

impl fmt::Debug for AltDirReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AltDirReg::AltDirect(alt_direct) => write!(f, "{:?}", alt_direct),
            AltDirReg::Register(register) => write!(f, "{:?}", register),
        }
    }
}

impl fmt::Display for AltDirReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AltDirReg::AltDirect(alt_direct) => alt_direct.fmt(f),
            AltDirReg::Register(register) => register.fmt(f),
        }
    }
}
