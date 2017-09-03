use std::collections::HashMap;
use pest::Error;
use var_instr::variable::{Variable, AsComplete, LabelNotFound};
use var_instr::variable::FromPair;
use machine::instruction::mem_size::MemSize;
use machine::instruction::parameter::{Indirect, Register, IndReg};
use label::Label;

#[derive(Debug)]
pub enum VarIndReg {
    Indirect(Variable<Indirect>),
    Register(Register),
}

impl MemSize for VarIndReg {
    fn mem_size(&self) -> usize {
        match *self {
            VarIndReg::Indirect(ref indirect) => indirect.mem_size(),
            VarIndReg::Register(register) => register.mem_size(),
        }
    }
}

impl FromPair for VarIndReg {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError> {
        match pair.as_rule() {
            ::Rule::indirect => Ok(VarIndReg::Indirect(Variable::from_pair(pair)?)),
            ::Rule::register => Ok(VarIndReg::Register(Register::from_pair(pair)?)),
            _ => Err(Error::CustomErrorSpan {
                message: format!("expected indirect, register found {:?}", pair.as_rule()),
                span: pair.clone().into_span(),
            }),
        }
    }
}

impl AsComplete<IndReg> for VarIndReg {
    fn as_complete(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<IndReg, LabelNotFound> {
        use self::VarIndReg::*;
        match *self {
            Indirect(ref indirect) => Ok(IndReg::Indirect(indirect.as_complete(offset, label_offsets)?)),
            Register(register) => Ok(IndReg::Register(register)),
        }
    }
}
