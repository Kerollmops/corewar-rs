use std::collections::HashMap;
use pest::Error;
use var_instr::variable::{Variable, AsComplete, LabelNotFound};
use var_instr::variable::FromPair;
use machine::instruction::mem_size::MemSize;
use machine::instruction::parameter::{AltDirect, Indirect, AltDirInd};
use label::Label;

#[derive(Debug)]
pub enum VarAltDirInd {
    AltDirect(Variable<AltDirect>),
    Indirect(Variable<Indirect>),
}

impl MemSize for VarAltDirInd {
    fn mem_size(&self) -> usize {
        match *self {
            VarAltDirInd::AltDirect(ref alt_direct) => alt_direct.mem_size(),
            VarAltDirInd::Indirect(ref indirect) => indirect.mem_size(),
        }
    }
}

impl FromPair for VarAltDirInd {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError> {
        match pair.as_rule() {
            ::Rule::direct => Ok(VarAltDirInd::AltDirect(Variable::from_pair(pair)?)),
            ::Rule::indirect => Ok(VarAltDirInd::Indirect(Variable::from_pair(pair)?)),
            _ => Err(Error::CustomErrorSpan {
                message: format!("expected direct, indirect found {:?}", pair.as_rule()),
                span: pair.clone().into_span(),
            }),
        }
    }
}

impl AsComplete<AltDirInd> for VarAltDirInd {
    fn as_complete(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<AltDirInd, LabelNotFound> {
        use self::VarAltDirInd::*;
        match *self {
            AltDirect(ref alt_direct) => Ok(AltDirInd::AltDirect(alt_direct.as_complete(offset, label_offsets)?)),
            Indirect(ref indirect) => Ok(AltDirInd::Indirect(indirect.as_complete(offset, label_offsets)?)),
        }
    }
}
