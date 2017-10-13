use std::io::{self, Read, Write};
use std::fmt;
use instruction::parameter::{AltDirect, Indirect};
use instruction::parameter::{ParamType, ParamTypeOf};
use instruction::parameter::InvalidParamType;
use instruction::parameter::ind_reg::Error;
use instruction::mem_size::MemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AltDirInd {
    AltDirect(AltDirect),
    Indirect(Indirect),
}

impl AltDirInd {
    pub fn read_from<R: Read>(param_type: ParamType, reader: &mut R) -> Result<Self, Error> {
        match param_type {
            ParamType::Direct => Ok(AltDirInd::AltDirect(AltDirect::read_from(reader)?)),
            ParamType::Indirect => Ok(AltDirInd::Indirect(Indirect::read_from(reader)?)),
            _ => Err(Error::InvalidParamType(InvalidParamType(param_type))),
        }
    }
}

impl GetValue for AltDirInd {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            AltDirInd::AltDirect(alt_direct) => alt_direct.get_value(vm, context),
            AltDirInd::Indirect(indirect) => indirect.get_value(vm, context),
        }
    }

    fn get_value_long(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            AltDirInd::AltDirect(alt_direct) => alt_direct.get_value_long(vm, context),
            AltDirInd::Indirect(indirect) => indirect.get_value_long(vm, context),
        }
    }
}

impl MemSize for AltDirInd {
    fn mem_size(&self) -> usize {
        match *self {
            AltDirInd::AltDirect(alt_direct) => alt_direct.mem_size(),
            AltDirInd::Indirect(indirect) => indirect.mem_size(),
        }
    }
}

impl WriteTo for AltDirInd {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match *self {
            AltDirInd::AltDirect(alt_direct) => alt_direct.write_to(writer),
            AltDirInd::Indirect(indirect) => indirect.write_to(writer),
        }
    }
}

impl ParamTypeOf for AltDirInd {
    fn param_type(&self) -> ParamType {
        match *self {
            AltDirInd::AltDirect(_) => ParamType::Direct,
            AltDirInd::Indirect(_) => ParamType::Indirect,
        }
    }
}

impl fmt::Debug for AltDirInd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AltDirInd::AltDirect(alt_direct) => write!(f, "{:?}", alt_direct),
            AltDirInd::Indirect(indirect) => write!(f, "{:?}", indirect),
        }
    }
}

impl fmt::Display for AltDirInd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AltDirInd::AltDirect(alt_direct) => alt_direct.fmt(f),
            AltDirInd::Indirect(indirect) => indirect.fmt(f),
        }
    }
}
