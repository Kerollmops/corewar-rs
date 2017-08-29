use var_instr::variable::Variable;
use machine::instruction::parameter::{Indirect, Register};

#[derive(Debug)]
pub enum VarIndReg {
    Indirect(Variable<Indirect>),
    Register(Register),
}
