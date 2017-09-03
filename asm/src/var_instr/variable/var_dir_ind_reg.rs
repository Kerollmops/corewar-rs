use std::collections::HashMap;
use pest::Error;
use var_instr::variable::{Variable, AsComplete, LabelNotFound};
use var_instr::variable::FromPair;
use machine::instruction::mem_size::MemSize;
use machine::instruction::parameter::{Direct, Indirect, Register, DirIndReg};
use label::Label;

#[derive(Debug)]
pub enum VarDirIndReg {
    Direct(Variable<Direct>),
    Indirect(Variable<Indirect>),
    Register(Register),
}

impl MemSize for VarDirIndReg {
    fn mem_size(&self) -> usize {
        match *self {
            VarDirIndReg::Direct(ref direct) => direct.mem_size(),
            VarDirIndReg::Indirect(ref indirect) => indirect.mem_size(),
            VarDirIndReg::Register(register) => register.mem_size(),
        }
    }
}

impl FromPair for VarDirIndReg {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError> {
        match pair.as_rule() {
            ::Rule::direct => Ok(VarDirIndReg::Direct(Variable::from_pair(pair)?)),
            ::Rule::indirect => Ok(VarDirIndReg::Indirect(Variable::from_pair(pair)?)),
            ::Rule::register => Ok(VarDirIndReg::Register(Register::from_pair(pair)?)),
            _ => Err(Error::CustomErrorSpan {
                message: format!("expected direct, indirect or register found {:?}", pair.as_rule()),
                span: pair.clone().into_span(),
            }),
        }
    }
}

impl AsComplete<DirIndReg> for VarDirIndReg {
    fn as_complete(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<DirIndReg, LabelNotFound> {
        use self::VarDirIndReg::*;
        match *self {
            Direct(ref direct) => Ok(DirIndReg::Direct(direct.as_complete(offset, label_offsets)?)),
            Indirect(ref indirect) => Ok(DirIndReg::Indirect(indirect.as_complete(offset, label_offsets)?)),
            Register(register) => Ok(DirIndReg::Register(register)),
        }
    }
}
