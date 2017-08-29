use var_instr::variable::Variable;
use machine::instruction::parameter::{Direct, Indirect, Register};

#[derive(Debug)]
pub enum VarDirIndReg {
    Direct(Variable<Direct>),
    Indirect(Variable<Indirect>),
    Register(Register),
}
