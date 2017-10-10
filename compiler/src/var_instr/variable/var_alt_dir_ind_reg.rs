use std::collections::HashMap;
use pest::Error;
use var_instr::variable::{Variable, AsComplete, LabelNotFound};
use var_instr::variable::FromPair;
use machine::instruction::mem_size::MemSize;
use machine::instruction::parameter::{AltDirect, Indirect, Register, AltDirIndReg};
use label::Label;

#[derive(Debug)]
pub enum VarAltDirIndReg {
    AltDirect(Variable<AltDirect>),
    Indirect(Variable<Indirect>),
    Register(Register),
}

impl MemSize for VarAltDirIndReg {
    fn mem_size(&self) -> usize {
        match *self {
            VarAltDirIndReg::AltDirect(ref alt_direct) => alt_direct.mem_size(),
            VarAltDirIndReg::Indirect(ref indirect) => indirect.mem_size(),
            VarAltDirIndReg::Register(register) => register.mem_size(),
        }
    }
}

impl FromPair for VarAltDirIndReg {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError> {
        match pair.as_rule() {
            ::Rule::direct => Ok(VarAltDirIndReg::AltDirect(Variable::from_pair(pair)?)),
            ::Rule::indirect => Ok(VarAltDirIndReg::Indirect(Variable::from_pair(pair)?)),
            ::Rule::register => Ok(VarAltDirIndReg::Register(Register::from_pair(pair)?)),
            _ => Err(Error::CustomErrorSpan {
                message: format!("expected direct, indirect or register found {:?}", pair.as_rule()),
                span: pair.clone().into_span(),
            }),
        }
    }
}

impl AsComplete<AltDirIndReg> for VarAltDirIndReg {
    fn as_complete(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<AltDirIndReg, LabelNotFound> {
        use self::VarAltDirIndReg::*;
        match *self {
            AltDirect(ref alt_direct) => Ok(AltDirIndReg::AltDirect(alt_direct.as_complete(offset, label_offsets)?)),
            Indirect(ref indirect) => Ok(AltDirIndReg::Indirect(indirect.as_complete(offset, label_offsets)?)),
            Register(register) => Ok(AltDirIndReg::Register(register)),
        }
    }
}
