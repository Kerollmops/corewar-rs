pub mod variable;

use machine::instruction::parameter::{Direct, Register};
use self::variable::*;

#[derive(Debug)]
pub enum VarInstr {
    Live(Variable<Direct>),
    Load(VarDirInd, Register),
    Store(Register, VarIndReg),
    Addition(Register, Register, Register),
    Substraction(Register, Register, Register),
    And(VarDirIndReg, VarDirIndReg, Register),
    Or(VarDirIndReg, VarDirIndReg, Register),
    Xor(VarDirIndReg, VarDirIndReg, Register),
    ZJump(Variable<Direct>),
    LoadIndex(VarDirIndReg, VarDirReg, Register),
    StoreIndex(Register, VarDirIndReg, VarDirReg),
    Fork(Variable<Direct>),
    LongLoad(VarDirInd, Register),
    LongLoadIndex(VarDirIndReg, VarDirReg, Register),
    Longfork(Variable<Direct>),
    Display(Register),
}
