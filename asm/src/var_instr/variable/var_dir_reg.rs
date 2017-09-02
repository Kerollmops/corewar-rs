use std::collections::HashMap;
use pest::Error;
use var_instr::variable::{Variable, AsComplete, LabelNotFound};
use var_instr::variable::FromPair;
use corewar::instruction::mem_size::MemSize;
use corewar::instruction::parameter::{Direct, Register, DirReg};
use label::Label;

#[derive(Debug)]
pub enum VarDirReg {
    Direct(Variable<Direct>),
    Register(Register),
}

impl MemSize for VarDirReg {
    fn mem_size(&self) -> usize {
        match *self {
            VarDirReg::Direct(ref direct) => direct.mem_size(),
            VarDirReg::Register(register) => register.mem_size(),
        }
    }
}

impl FromPair for VarDirReg {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError> {
        match pair.as_rule() {
            ::Rule::direct => Ok(VarDirReg::Direct(Variable::from_pair(pair)?)),
            ::Rule::register => Ok(VarDirReg::Register(Register::from_pair(pair)?)),
            _ => Err(Error::CustomErrorSpan {
                message: format!("expected direct, register found {:?}", pair.as_rule()),
                span: pair.clone().into_span(),
            }),
        }
    }
}

impl AsComplete<DirReg> for VarDirReg {
    fn as_complete(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<DirReg, LabelNotFound> {
        use self::VarDirReg::*;
        match *self {
            Direct(ref direct) => Ok(DirReg::Direct(direct.as_complete(offset, label_offsets)?)),
            Register(register) => Ok(DirReg::Register(register)),
        }
    }
}
