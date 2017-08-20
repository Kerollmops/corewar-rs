use std::io::Read;
use std::convert::TryFrom;
use instruction::parameter::{Direct, Indirect, ParamType};
use instruction::parameter::{Register, InvalidRegister};
use instruction::mem_size::MemSize;
use instruction::get_value::GetValue;
use virtual_machine::VirtualMachine;
use process::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirIndReg {
    Direct(Direct),
    Indirect(Indirect),
    Register(Register),
}

impl GetValue for DirIndReg {
    fn get_value(&self, vm: &VirtualMachine, context: &Context) -> i32 {
        match *self {
            DirIndReg::Direct(direct) => direct.get_value(vm, context),
            DirIndReg::Indirect(indirect) => indirect.get_value(vm, context),
            DirIndReg::Register(register) => register.get_value(vm, context),
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

impl<'a, R: Read> TryFrom<(ParamType, &'a mut R)> for DirIndReg {
    type Error = InvalidRegister;

    fn try_from((param_type, reader): (ParamType, &'a mut R)) -> Result<Self, Self::Error> {
        match param_type {
            ParamType::Direct => Ok(DirIndReg::Direct(Direct::from(reader))),
            ParamType::Indirect => Ok(DirIndReg::Indirect(Indirect::from(reader))),
            ParamType::Register => Ok(DirIndReg::Register(Register::try_from(reader)?)),
        }
    }
}
