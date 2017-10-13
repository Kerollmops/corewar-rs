use std::io::{self, Read, Write};
use std::fmt;
use instruction::parameter::{AltDirect, Indirect};
use instruction::parameter::{ParamType, ParamTypeOf};
use instruction::parameter::Register;
use instruction::parameter::dir_ind_reg::Error;
use instruction::mem_size::MemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AltDirIndReg {
    AltDirect(AltDirect),
    Indirect(Indirect),
    Register(Register),
}

impl AltDirIndReg {
    pub fn read_from<R: Read>(param_type: ParamType, reader: &mut R) -> Result<Self, Error> {
        match param_type {
            ParamType::Direct => Ok(AltDirIndReg::AltDirect(AltDirect::read_from(reader)?)),
            ParamType::Indirect => Ok(AltDirIndReg::Indirect(Indirect::read_from(reader)?)),
            ParamType::Register => Ok(AltDirIndReg::Register(Register::read_from(reader)?)),
        }
    }
}

impl GetValue for AltDirIndReg {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            AltDirIndReg::AltDirect(alt_direct) => alt_direct.get_value(vm, context),
            AltDirIndReg::Indirect(indirect) => indirect.get_value(vm, context),
            AltDirIndReg::Register(register) => register.get_value(vm, context),
        }
    }

    fn get_value_long(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            AltDirIndReg::AltDirect(alt_direct) => alt_direct.get_value_long(vm, context),
            AltDirIndReg::Indirect(indirect) => indirect.get_value_long(vm, context),
            AltDirIndReg::Register(register) => register.get_value_long(vm, context),
        }
    }
}

impl MemSize for AltDirIndReg {
    fn mem_size(&self) -> usize {
        match *self {
            AltDirIndReg::AltDirect(alt_direct) => alt_direct.mem_size(),
            AltDirIndReg::Indirect(indirect) => indirect.mem_size(),
            AltDirIndReg::Register(register) => register.mem_size(),
        }
    }
}

impl ParamTypeOf for AltDirIndReg {
    fn param_type(&self) -> ParamType {
        match *self {
            AltDirIndReg::AltDirect(_) => ParamType::Direct,
            AltDirIndReg::Indirect(_) => ParamType::Indirect,
            AltDirIndReg::Register(_) => ParamType::Register,
        }
    }
}

impl WriteTo for AltDirIndReg {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match *self {
            AltDirIndReg::AltDirect(alt_direct) => alt_direct.write_to(writer),
            AltDirIndReg::Indirect(indirect) => indirect.write_to(writer),
            AltDirIndReg::Register(register) => register.write_to(writer),
        }
    }
}

impl fmt::Debug for AltDirIndReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AltDirIndReg::AltDirect(alt_direct) => write!(f, "{:?}", alt_direct),
            AltDirIndReg::Indirect(indirect) => write!(f, "{:?}", indirect),
            AltDirIndReg::Register(register) => write!(f, "{:?}", register),
        }
    }
}

impl fmt::Display for AltDirIndReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AltDirIndReg::AltDirect(alt_direct) => alt_direct.fmt(f),
            AltDirIndReg::Indirect(indirect) => indirect.fmt(f),
            AltDirIndReg::Register(register) => register.fmt(f),
        }
    }
}
