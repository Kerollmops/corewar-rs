use var_instr::variable::Variable;
use machine::instruction::parameter::{Direct, Indirect};

#[derive(Debug)]
pub enum VarDirInd {
    Direct(Variable<Direct>),
    Indirect(Variable<Indirect>),
}
