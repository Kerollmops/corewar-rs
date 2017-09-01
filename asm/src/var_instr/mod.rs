pub mod variable;

use std::convert::TryFrom;
use std::collections::HashMap;
use pest::Error;
use machine::instruction::Instruction;
use machine::instruction::mem_size::MemSize;
use machine::instruction::{OP_CODE_SIZE, PARAM_CODE_SIZE};
use machine::instruction::parameter::*;
use self::variable::*;
use label::Label;
use ::{Rule, AsmPair, AsmError};

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
    LongFork(Variable<Direct>),
    Display(Register),
}

impl VarInstr {
    pub fn as_instr(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<Instruction, LabelNotFound> {
        use self::VarInstr::*;
        match *self {
            Live(ref direct) => Ok(Instruction::Live(direct.as_complete(offset, label_offsets)?)),
            Load(ref var_dir_ind, reg) => Ok(Instruction::Load(var_dir_ind.as_complete(offset, label_offsets)?, reg)),
            Store(reg, ref ind_reg) => Ok(Instruction::Store(reg, ind_reg.as_complete(offset, label_offsets)?)),
            Addition(a, b, c) => Ok(Instruction::Addition(a, b, c)),
            Substraction(a, b, c) => Ok(Instruction::Substraction(a, b, c)),
            And(ref dir_ind_reg_a, ref dir_ind_reg_b, reg) => {
                let dir_ind_reg_a = dir_ind_reg_a.as_complete(offset, label_offsets)?;
                let dir_ind_reg_b = dir_ind_reg_b.as_complete(offset, label_offsets)?;
                Ok(Instruction::And(dir_ind_reg_a, dir_ind_reg_b, reg))
            },
            Or(ref dir_ind_reg_a, ref dir_ind_reg_b, reg) => {
                let dir_ind_reg_a = dir_ind_reg_a.as_complete(offset, label_offsets)?;
                let dir_ind_reg_b = dir_ind_reg_b.as_complete(offset, label_offsets)?;
                Ok(Instruction::Or(dir_ind_reg_a, dir_ind_reg_b, reg))
            },
            Xor(ref dir_ind_reg_a, ref dir_ind_reg_b, reg) => {
                let dir_ind_reg_a = dir_ind_reg_a.as_complete(offset, label_offsets)?;
                let dir_ind_reg_b = dir_ind_reg_b.as_complete(offset, label_offsets)?;
                Ok(Instruction::Xor(dir_ind_reg_a, dir_ind_reg_b, reg))
            },
            ZJump(ref direct) => Ok(Instruction::ZJump(direct.as_complete(offset, label_offsets)?)),
            LoadIndex(ref dir_ind_reg, ref dir_reg, reg) => {
                let dir_ind_reg = dir_ind_reg.as_complete(offset, label_offsets)?;
                let dir_reg = dir_reg.as_complete(offset, label_offsets)?;
                Ok(Instruction::LoadIndex(dir_ind_reg, dir_reg, reg))
            },
            StoreIndex(reg, ref dir_ind_reg, ref dir_reg) => {
                let dir_ind_reg = dir_ind_reg.as_complete(offset, label_offsets)?;
                let dir_reg = dir_reg.as_complete(offset, label_offsets)?;
                Ok(Instruction::StoreIndex(reg, dir_ind_reg, dir_reg))
            },
            Fork(ref direct) => Ok(Instruction::Fork(direct.as_complete(offset, label_offsets)?)),
            LongLoad(ref dir_ind, reg) => {
                let dir_ind = dir_ind.as_complete(offset, label_offsets)?;
                Ok(Instruction::LongLoad(dir_ind, reg))
            },
            LongLoadIndex(ref dir_ind_reg, ref dir_reg, reg) => {
                let dir_ind_reg = dir_ind_reg.as_complete(offset, label_offsets)?;
                let dir_reg = dir_reg.as_complete(offset, label_offsets)?;
                Ok(Instruction::LongLoadIndex(dir_ind_reg, dir_reg, reg))
            },
            LongFork(ref direct) => Ok(Instruction::LongFork(direct.as_complete(offset, label_offsets)?)),
            Display(reg) => Ok(Instruction::Display(reg)),
        }
    }
}

impl MemSize for VarInstr {
    fn mem_size(&self) -> usize {
        use self::VarInstr::*;
        let size = match *self {
            Live(ref a) => a.mem_size(),
            Load(ref a, b) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size(),
            Store(a, ref b) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size(),
            Addition(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            Substraction(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            And(ref a, ref b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            Or(ref a, ref b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            Xor(ref a, ref b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            ZJump(ref a) => a.mem_size(),
            LoadIndex(ref a, ref b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            StoreIndex(ref a, ref b, ref c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            Fork(ref a) => a.mem_size(),
            LongLoad(ref a, b) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size(),
            LongLoadIndex(ref a, ref b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            LongFork(ref a) => a.mem_size(),
            Display(a) => PARAM_CODE_SIZE + a.mem_size(),
        };
        OP_CODE_SIZE + size
    }
}

impl TryFrom<AsmPair> for VarInstr {
    type Error = AsmError;

    fn try_from(instr: AsmPair) -> Result<Self, Self::Error> {
        let instr_span = instr.clone().into_span();
        let mut instr = instr.into_inner();
        let name = instr.by_ref().find(|p| p.as_rule() == Rule::instr_name).unwrap().into_span();
        match name.as_str() {
            "live" => match instr.next() {
                Some(pair) => Ok(VarInstr::Live(Variable::from_pair(pair)?)),
                None => Err(Error::CustomErrorSpan {
                    message: "missing parameter".into(),
                    span: instr_span.clone(),
                }),
            },
            "ld" => match (instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b)) => {
                    let var_dir_ind = VarDirInd::from_pair(pair_a)?;
                    let reg = Register::from_pair(pair_b)?;
                    Ok(VarInstr::Load(var_dir_ind, reg))
                },
                (_, _) => Err(Error::CustomErrorSpan {
                    message: "expected two parameters".into(),
                    span: instr_span.clone()
                })
            },
            "st" => match (instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b)) => {
                    let reg = Register::from_pair(pair_a)?;
                    let ind_reg = VarIndReg::from_pair(pair_b)?;
                    Ok(VarInstr::Store(reg, ind_reg))
                },
                (_, _) => Err(Error::CustomErrorSpan {
                    message: "expected two parameters".into(),
                    span: instr_span.clone()
                })
            },
            "add" => match (instr.next(), instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b), Some(pair_c)) => {
                    let reg_a = Register::from_pair(pair_a)?;
                    let reg_b = Register::from_pair(pair_b)?;
                    let reg_c = Register::from_pair(pair_c)?;
                    Ok(VarInstr::Addition(reg_a, reg_b, reg_c))
                },
                (_, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                })
            },
            "sub" => match (instr.next(), instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b), Some(pair_c)) => {
                    let reg_a = Register::from_pair(pair_a)?;
                    let reg_b = Register::from_pair(pair_b)?;
                    let reg_c = Register::from_pair(pair_c)?;
                    Ok(VarInstr::Substraction(reg_a, reg_b, reg_c))
                },
                (_, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                })
            },
            "and" => match (instr.next(), instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b), Some(pair_c)) => {
                    let dir_ind_reg_a = VarDirIndReg::from_pair(pair_a)?;
                    let dir_ind_reg_b = VarDirIndReg::from_pair(pair_b)?;
                    let reg_c = Register::from_pair(pair_c)?;
                    Ok(VarInstr::And(dir_ind_reg_a, dir_ind_reg_b, reg_c))
                },
                (_, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                })
            },
            "or" => match (instr.next(), instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b), Some(pair_c)) => {
                    let dir_ind_reg_a = VarDirIndReg::from_pair(pair_a)?;
                    let dir_ind_reg_b = VarDirIndReg::from_pair(pair_b)?;
                    let reg_c = Register::from_pair(pair_c)?;
                    Ok(VarInstr::Or(dir_ind_reg_a, dir_ind_reg_b, reg_c))
                },
                (_, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                })
            },
            "xor" => match (instr.next(), instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b), Some(pair_c)) => {
                    let dir_ind_reg_a = VarDirIndReg::from_pair(pair_a)?;
                    let dir_ind_reg_b = VarDirIndReg::from_pair(pair_b)?;
                    let reg_c = Register::from_pair(pair_c)?;
                    Ok(VarInstr::Xor(dir_ind_reg_a, dir_ind_reg_b, reg_c))
                },
                (_, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                })
            },
            "zjump" => match instr.next() {
                Some(pair) => Ok(VarInstr::ZJump(Variable::from_pair(pair)?)),
                None => Err(Error::CustomErrorSpan {
                    message: "missing parameter".into(),
                    span: instr_span.clone(),
                }),
            },
            "ldi" => match (instr.next(), instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b), Some(pair_c)) => {
                    let dir_ind_reg = VarDirIndReg::from_pair(pair_a)?;
                    let dir_reg = VarDirReg::from_pair(pair_b)?;
                    let reg = Register::from_pair(pair_c)?;
                    Ok(VarInstr::LoadIndex(dir_ind_reg, dir_reg, reg))
                },
                (_, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                })
            },
            "sti" => match (instr.next(), instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b), Some(pair_c)) => {
                    let reg = Register::from_pair(pair_a)?;
                    let dir_ind_reg = VarDirIndReg::from_pair(pair_b)?;
                    let dir_reg = VarDirReg::from_pair(pair_c)?;
                    Ok(VarInstr::StoreIndex(reg, dir_ind_reg, dir_reg))
                },
                (_, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                })
            },
            "fork" => match instr.next() {
                Some(pair) => Ok(VarInstr::Fork(Variable::from_pair(pair)?)),
                None => Err(Error::CustomErrorSpan {
                    message: "missing parameter".into(),
                    span: instr_span.clone(),
                }),
            },
            "lld" => match (instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b)) => {
                    let dir_ind = VarDirInd::from_pair(pair_a)?;
                    let reg = Register::from_pair(pair_b)?;
                    Ok(VarInstr::LongLoad(dir_ind, reg))
                },
                (_, _) => Err(Error::CustomErrorSpan {
                    message: "expected two parameters".into(),
                    span: instr_span.clone()
                })
            },
            "lldi" => match (instr.next(), instr.next(), instr.next()) {
                (Some(pair_a), Some(pair_b), Some(pair_c)) => {
                    let dir_ind_reg = VarDirIndReg::from_pair(pair_a)?;
                    let dir_reg = VarDirReg::from_pair(pair_b)?;
                    let reg = Register::from_pair(pair_c)?;
                    Ok(VarInstr::LongLoadIndex(dir_ind_reg, dir_reg, reg))
                },
                (_, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                })
            },
            "lfork" => match instr.next() {
                Some(pair) => Ok(VarInstr::LongFork(Variable::from_pair(pair)?)),
                None => Err(Error::CustomErrorSpan {
                    message: "missing parameter".into(),
                    span: instr_span.clone(),
                }),
            },
            "aff" | "disp" => match instr.next() {
                Some(pair) => Ok(VarInstr::Display(Register::from_pair(pair)?)),
                None => Err(Error::CustomErrorSpan {
                    message: "missing parameter".into(),
                    span: instr_span.clone(),
                })
            },
            instr_name => Err(Error::CustomErrorSpan {
                message: format!("unknown instruction {}", instr_name),
                span: name.clone()
            }),
        }
    }
}
