use std::collections::HashMap;
use pest::Error;
use var_instr::variable::{Variable, AsComplete, LabelNotFound};
use var_instr::variable::FromPair;
use machine::instruction::mem_size::MemSize;
use machine::instruction::parameter::{AltDirect, Register, AltDirReg};
use label::Label;

#[derive(Debug)]
pub enum VarAltDirReg {
    AltDirect(Variable<AltDirect>),
    Register(Register),
}

impl MemSize for VarAltDirReg {
    fn mem_size(&self) -> usize {
        match *self {
            VarAltDirReg::AltDirect(ref alt_direct) => alt_direct.mem_size(),
            VarAltDirReg::Register(register) => register.mem_size(),
        }
    }
}

impl FromPair for VarAltDirReg {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError> {
        match pair.as_rule() {
            ::Rule::direct => Ok(VarAltDirReg::AltDirect(Variable::from_pair(pair)?)),
            ::Rule::register => Ok(VarAltDirReg::Register(Register::from_pair(pair)?)),
            _ => Err(Error::CustomErrorSpan {
                message: format!("expected direct, register found {:?}", pair.as_rule()),
                span: pair.clone().into_span(),
            }),
        }
    }
}

impl AsComplete<AltDirReg> for VarAltDirReg {
    fn as_complete(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<AltDirReg, LabelNotFound> {
        use self::VarAltDirReg::*;
        match *self {
            AltDirect(ref alt_direct) => Ok(AltDirReg::AltDirect(alt_direct.as_complete(offset, label_offsets)?)),
            Register(register) => Ok(AltDirReg::Register(register)),
        }
    }
}
