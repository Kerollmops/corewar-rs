use std::io::{Read, Write};
use std::convert::TryFrom;
use instruction::parameter::{Direct, Indirect};
use instruction::parameter::{ParamType, ParamTypeOf};
use instruction::mem_size::MemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Debug, Clone, Copy)]
pub struct InvalidParamType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirInd {
    Direct(Direct),
    Indirect(Indirect),
}

impl GetValue for DirInd {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            DirInd::Direct(direct) => direct.get_value(vm, context),
            DirInd::Indirect(indirect) => indirect.get_value(vm, context),
        }
    }

    fn get_value_long(&self, vm: &Machine, context: &Context) -> i32 {
        match *self {
            DirInd::Direct(direct) => direct.get_value_long(vm, context),
            DirInd::Indirect(indirect) => indirect.get_value_long(vm, context),
        }
    }
}

impl MemSize for DirInd {
    fn mem_size(&self) -> usize {
        match *self {
            DirInd::Direct(direct) => direct.mem_size(),
            DirInd::Indirect(indirect) => indirect.mem_size(),
        }
    }
}

impl WriteTo for DirInd {
    fn write_to<W: Write>(&self, writer: &mut W) {
        match *self {
            DirInd::Direct(direct) => direct.write_to(writer),
            DirInd::Indirect(indirect) => indirect.write_to(writer),
        }
    }
}

impl ParamTypeOf for DirInd {
    fn param_type(&self) -> ParamType {
        match *self {
            DirInd::Direct(_) => ParamType::Direct,
            DirInd::Indirect(_) => ParamType::Indirect,
        }
    }
}

impl<'a, R: Read> TryFrom<(ParamType, &'a mut R)> for DirInd {
    type Error = InvalidParamType;

    fn try_from((param_type, reader): (ParamType, &'a mut R)) -> Result<Self, Self::Error> {
        match param_type {
            ParamType::Direct => Ok(DirInd::Direct(Direct::from(reader))),
            ParamType::Indirect => Ok(DirInd::Indirect(Indirect::from(reader))),
            _ => Err(InvalidParamType),
        }
    }
}
