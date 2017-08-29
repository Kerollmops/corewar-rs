use var_instr::variable::Variable;
use machine::instruction::parameter::{Direct, Register};

#[derive(Debug)]
pub enum VarDirReg {
    Direct(Variable<Direct>),
    Register(Variable<Register>),
}
