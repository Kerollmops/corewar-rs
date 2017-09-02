use std::collections::HashMap;
use pest::Error;
use var_instr::variable::{Variable, AsComplete, LabelNotFound};
use var_instr::variable::FromPair;
use corewar::instruction::mem_size::MemSize;
use corewar::instruction::parameter::{Direct, Indirect, DirInd};
use label::Label;

#[derive(Debug)]
pub enum VarDirInd {
    Direct(Variable<Direct>),
    Indirect(Variable<Indirect>),
}

impl MemSize for VarDirInd {
    fn mem_size(&self) -> usize {
        match *self {
            VarDirInd::Direct(ref direct) => direct.mem_size(),
            VarDirInd::Indirect(ref indirect) => indirect.mem_size(),
        }
    }
}

impl FromPair for VarDirInd {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError> {
        match pair.as_rule() {
            ::Rule::direct => Ok(VarDirInd::Direct(Variable::from_pair(pair)?)),
            ::Rule::indirect => Ok(VarDirInd::Indirect(Variable::from_pair(pair)?)),
            _ => Err(Error::CustomErrorSpan {
                message: format!("expected direct, indirect found {:?}", pair.as_rule()),
                span: pair.clone().into_span(),
            }),
        }
    }
}

impl AsComplete<DirInd> for VarDirInd {
    fn as_complete(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<DirInd, LabelNotFound> {
        use self::VarDirInd::*;
        match *self {
            Direct(ref direct) => Ok(DirInd::Direct(direct.as_complete(offset, label_offsets)?)),
            Indirect(ref indirect) => Ok(DirInd::Indirect(indirect.as_complete(offset, label_offsets)?)),
        }
    }
}
