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
use ::{AsmPair, AsmError};

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
    ZJump(Variable<AltDirect>),
    LoadIndex(VarAltDirIndReg, VarAltDirReg, Register),
    StoreIndex(Register, VarAltDirIndReg, VarAltDirReg),
    Fork(Variable<AltDirect>),
    LongLoad(VarDirInd, Register),
    LongLoadIndex(VarAltDirIndReg, VarAltDirReg, Register),
    LongFork(Variable<AltDirect>),
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
            ZJump(ref alt_direct) => Ok(Instruction::ZJump(alt_direct.as_complete(offset, label_offsets)?)),
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

impl HasParamCode for VarInstr {
    fn has_param_code(&self) -> bool {
        use self::VarInstr::*;
        match *self {
            Live(_) => false,
            Load(_, _) => true,
            Store(_, _) => true,
            Addition(_, _, _) => false,
            Substraction(_, _, _) => false,
            And(_, _, _) => true,
            Or(_, _, _) => true,
            Xor(_, _, _) => true,
            ZJump(_) => false,
            LoadIndex(_, _, _) => true,
            StoreIndex(_, _, _) => true,
            Fork(_) => false,
            LongLoad(_, _) => true,
            LongLoadIndex(_, _, _) => true,
            LongFork(_) => false,
            Display(_) => false,
        }
    }
}

impl MemSize for VarInstr {
    fn mem_size(&self) -> usize {
        use self::VarInstr::*;
        let size = match *self {
            Live(ref a) => a.mem_size(),
            Load(ref a, b) => a.mem_size() + b.mem_size(),
            Store(a, ref b) => a.mem_size() + b.mem_size(),
            Addition(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            Substraction(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            And(ref a, ref b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            Or(ref a, ref b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            Xor(ref a, ref b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            ZJump(ref a) => a.mem_size(),
            LoadIndex(ref a, ref b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            StoreIndex(ref a, ref b, ref c) => a.mem_size() + b.mem_size() + c.mem_size(),
            Fork(ref a) => a.mem_size(),
            LongLoad(ref a, b) => a.mem_size() + b.mem_size(),
            LongLoadIndex(ref a, ref b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            LongFork(ref a) => a.mem_size(),
            Display(a) => a.mem_size(),
        };
        let param_code = if self.has_param_code() { PARAM_CODE_SIZE } else { 0 };
        OP_CODE_SIZE + param_code + size
    }
}

macro_rules! next_param {
    ($instr:ident) => {
        $instr.next().and_then(|p| p.into_inner().next())
    };
}

impl TryFrom<AsmPair> for VarInstr {
    type Error = AsmError;

    fn try_from(instr: AsmPair) -> Result<Self, Self::Error> {
        let instr_span = instr.clone().into_span();
        let mut instr = instr.into_inner();
        let name = instr.by_ref().next().unwrap().into_inner().next().unwrap().into_span();
        match name.as_str() {
            "live" => match (next_param!(instr), next_param!(instr)) {
                (Some(pair), None) => Ok(VarInstr::Live(Variable::from_pair(pair)?)),
                _ => Err(Error::CustomErrorSpan {
                    message: "expected one parameter".into(),
                    span: instr_span.clone(),
                }),
            },
            "ld" => match (next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), None) => {
                    let var_dir_ind = VarDirInd::from_pair(pair_a)?;
                    let reg = Register::from_pair(pair_b)?;
                    Ok(VarInstr::Load(var_dir_ind, reg))
                },
                _ => Err(Error::CustomErrorSpan {
                    message: "expected two parameters".into(),
                    span: instr_span.clone()
                })
            },
            "st" => match (next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), None) => {
                    let reg = Register::from_pair(pair_a)?;
                    let ind_reg = VarIndReg::from_pair(pair_b)?;
                    Ok(VarInstr::Store(reg, ind_reg))
                },
                _ => Err(Error::CustomErrorSpan {
                    message: "expected two parameters".into(),
                    span: instr_span.clone()
                })
            },
            "add" => match (next_param!(instr), next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), Some(pair_c), None) => {
                    let reg_a = Register::from_pair(pair_a)?;
                    let reg_b = Register::from_pair(pair_b)?;
                    let reg_c = Register::from_pair(pair_c)?;
                    Ok(VarInstr::Addition(reg_a, reg_b, reg_c))
                },
                (_, _, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                }),
            },
            "sub" => match (next_param!(instr), next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), Some(pair_c), None) => {
                    let reg_a = Register::from_pair(pair_a)?;
                    let reg_b = Register::from_pair(pair_b)?;
                    let reg_c = Register::from_pair(pair_c)?;
                    Ok(VarInstr::Substraction(reg_a, reg_b, reg_c))
                },
                (_, _, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                }),
            },
            "and" => match (next_param!(instr), next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), Some(pair_c), None) => {
                    let dir_ind_reg_a = VarDirIndReg::from_pair(pair_a)?;
                    let dir_ind_reg_b = VarDirIndReg::from_pair(pair_b)?;
                    let reg_c = Register::from_pair(pair_c)?;
                    Ok(VarInstr::And(dir_ind_reg_a, dir_ind_reg_b, reg_c))
                },
                (_, _, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                }),
            },
            "or" => match (next_param!(instr), next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), Some(pair_c), None) => {
                    let dir_ind_reg_a = VarDirIndReg::from_pair(pair_a)?;
                    let dir_ind_reg_b = VarDirIndReg::from_pair(pair_b)?;
                    let reg_c = Register::from_pair(pair_c)?;
                    Ok(VarInstr::Or(dir_ind_reg_a, dir_ind_reg_b, reg_c))
                },
                (_, _, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                }),
            },
            "xor" => match (next_param!(instr), next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), Some(pair_c), None) => {
                    let dir_ind_reg_a = VarDirIndReg::from_pair(pair_a)?;
                    let dir_ind_reg_b = VarDirIndReg::from_pair(pair_b)?;
                    let reg_c = Register::from_pair(pair_c)?;
                    Ok(VarInstr::Xor(dir_ind_reg_a, dir_ind_reg_b, reg_c))
                },
                (_, _, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                }),
            },
            "zjmp" => match (next_param!(instr), next_param!(instr)) {
                (Some(pair), None) => Ok(VarInstr::ZJump(Variable::from_pair(pair)?)),
                _ => Err(Error::CustomErrorSpan {
                    message: "expected one parameter".into(),
                    span: instr_span.clone(),
                }),
            },
            "ldi" => match (next_param!(instr), next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), Some(pair_c), None) => {
                    let alt_dir_ind_reg = VarAltDirIndReg::from_pair(pair_a)?;
                    let alt_dir_reg = VarAltDirReg::from_pair(pair_b)?;
                    let reg = Register::from_pair(pair_c)?;
                    Ok(VarInstr::LoadIndex(alt_dir_ind_reg, alt_dir_reg, reg))
                },
                (_, _, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                }),
            },
            "sti" => match (next_param!(instr), next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), Some(pair_c), None) => {
                    let reg = Register::from_pair(pair_a)?;
                    let alt_dir_ind_reg = VarAltDirIndReg::from_pair(pair_b)?;
                    let alt_dir_reg = VarAltDirReg::from_pair(pair_c)?;
                    Ok(VarInstr::StoreIndex(reg, alt_dir_ind_reg, alt_dir_reg))
                },
                (_, _, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                }),
            },
            "fork" => match (next_param!(instr), next_param!(instr)) {
                (Some(pair), None) => Ok(VarInstr::Fork(Variable::from_pair(pair)?)),
                _ => Err(Error::CustomErrorSpan {
                    message: "expected one parameter".into(),
                    span: instr_span.clone(),
                }),
            },
            "lld" => match (next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), None) => {
                    let dir_ind = VarDirInd::from_pair(pair_a)?;
                    let reg = Register::from_pair(pair_b)?;
                    Ok(VarInstr::LongLoad(dir_ind, reg))
                },
                _ => Err(Error::CustomErrorSpan {
                    message: "expected two parameters".into(),
                    span: instr_span.clone()
                })
            },
            "lldi" => match (next_param!(instr), next_param!(instr), next_param!(instr), next_param!(instr)) {
                (Some(pair_a), Some(pair_b), Some(pair_c), None) => {
                    let alt_dir_ind_reg = VarAltDirIndReg::from_pair(pair_a)?;
                    let alt_dir_reg = VarAltDirReg::from_pair(pair_b)?;
                    let reg = Register::from_pair(pair_c)?;
                    Ok(VarInstr::LongLoadIndex(alt_dir_ind_reg, alt_dir_reg, reg))
                },
                (_, _, _, _) => Err(Error::CustomErrorSpan {
                    message: "expected three parameters".into(),
                    span: instr_span.clone()
                }),
            },
            "lfork" => match (next_param!(instr), next_param!(instr)) {
                (Some(pair), None) => Ok(VarInstr::LongFork(Variable::from_pair(pair)?)),
                _ => Err(Error::CustomErrorSpan {
                    message: "expected one parameter".into(),
                    span: instr_span.clone(),
                }),
            },
            "aff" | "disp" => match (next_param!(instr), next_param!(instr)) {
                (Some(pair), None) => Ok(VarInstr::Display(Register::from_pair(pair)?)),
                _ => Err(Error::CustomErrorSpan {
                    message: "expected one parameter".into(),
                    span: instr_span.clone(),
                }),
            },
            instr_name => Err(Error::CustomErrorSpan {
                message: format!("unknown instruction {}", instr_name),
                span: name.clone(),
            }),
        }
    }
}
