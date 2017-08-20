mod parameter;
mod mem_size;

use std::io::Read;
use std::convert::TryFrom;
use byteorder::ReadBytesExt;
use self::parameter::*;
use self::mem_size::MemSize;

const OP_CODE_SIZE:     usize = 1;
const PARAM_CODE_SIZE:  usize = 1;

use self::Instruction::*;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    NoOp,
    Live(Direct),
    Load(DirInd, Register),
    Store(Register, IndReg),
    Addition(Register, Register, Register),
    Substraction(Register, Register, Register),
    And(DirIndReg, DirIndReg, Register),
    Or(DirIndReg, DirIndReg, Register),
    Xor(DirIndReg, DirIndReg, Register),
    ZJump(Direct),
    LoadIndex(DirIndReg, DirReg, Register),
    StoreIndex(Register, DirIndReg, DirReg),
    Fork(Direct),
    LongLoad(DirInd, Register),
    LongLoadIndex(DirIndReg, DirReg, Register),
    Longfork(Direct),
    Display(Register),
}

impl Instruction {
    pub fn cycle_cost(&self) -> usize {
        match *self {
            NoOp => 1,
            Live(_) => 10,
            Load(_, _) => 5,
            Store(_, _) => 5,
            Addition(_, _, _) => 10,
            Substraction(_, _, _) => 10,
            And(_, _, _) => 6,
            Or(_, _, _) => 6,
            Xor(_, _, _) => 6,
            ZJump(_) => 20,
            LoadIndex(_, _, _) => 25,
            StoreIndex(_, _, _) => 25,
            Fork(_) => 800,
            LongLoad(_, _) => 10,
            LongLoadIndex(_, _, _) => 50,
            Longfork(_) => 1000,
            Display(_) => 2,
        }
    }
}

impl MemSize for Instruction {
    fn mem_size(&self) -> usize {
        let size = match *self {
            NoOp => 0,
            Live(a) => a.mem_size(),
            Load(a, b) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size(),
            Store(a, b) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size(),
            Addition(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            Substraction(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            And(a, b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            Or(a, b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            Xor(a, b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            ZJump(a) => a.mem_size(),
            LoadIndex(a, b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            StoreIndex(a, b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            Fork(a) => a.mem_size(),
            LongLoad(a, b) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size(),
            LongLoadIndex(a, b, c) => PARAM_CODE_SIZE + a.mem_size() + b.mem_size() + c.mem_size(),
            Longfork(a) => a.mem_size(),
            Display(a) => a.mem_size(),
        };
        OP_CODE_SIZE + size
    }
}

macro_rules! try_param_type {
    ($n:expr, $r:expr) => ({
        use self::ParamNumber::*;
        match ParamCode::from(&mut $r).param_type_of($n) {
            Ok(param_code) => param_code,
            Err(_) => return NoOp,
        }
    });
}

impl<R: Read> From<R> for Instruction {
    fn from(mut reader: R) -> Self {
        match reader.read_u8().unwrap() {
            1 => Live(Direct::from(&mut reader)),
            2 => {
                let param_code = try_param_type!(First, reader);
                match (DirInd::try_from((param_code, &mut reader)), Register::try_from(&mut reader)) {
                    (Ok(dir_ind), Ok(reg)) => Load(dir_ind, reg),
                    _ => NoOp,
                }
            },
            3 => {
                let param_code = try_param_type!(Second, reader);
                match (Register::try_from(&mut reader), IndReg::try_from((param_code, &mut reader))) {
                    (Ok(reg), Ok(ind_reg)) => Store(reg, ind_reg),
                    _ => NoOp,
                }
            },
            4 => {
                let reg_a = Register::try_from(&mut reader);
                let reg_b = Register::try_from(&mut reader);
                let reg_c = Register::try_from(&mut reader);
                match (reg_a, reg_b, reg_c) {
                    (Ok(a), Ok(b), Ok(c)) => Addition(a, b, c),
                    _ => NoOp,
                }
            },
            5 => {
                let reg_a = Register::try_from(&mut reader);
                let reg_b = Register::try_from(&mut reader);
                let reg_c = Register::try_from(&mut reader);
                match (reg_a, reg_b, reg_c) {
                    (Ok(a), Ok(b), Ok(c)) => Substraction(a, b, c),
                    _ => NoOp,
                }
            },
            6 => {
                let first_type = try_param_type!(First, reader);
                let second_type = try_param_type!(Second, reader);

                let dir_ind_reg_a = DirIndReg::try_from((first_type, &mut reader));
                let dir_ind_reg_b = DirIndReg::try_from((second_type, &mut reader));
                let reg = Register::try_from(&mut reader);

                match (dir_ind_reg_a, dir_ind_reg_b, reg) {
                    (Ok(dir_ind_reg_a), Ok(dir_ind_reg_b), Ok(reg)) => And(dir_ind_reg_a, dir_ind_reg_b, reg),
                    _ => NoOp,
                }
            },
            7 => {
                let first_type = try_param_type!(First, reader);
                let second_type = try_param_type!(Second, reader);

                let dir_ind_reg_a = DirIndReg::try_from((first_type, &mut reader));
                let dir_ind_reg_b = DirIndReg::try_from((second_type, &mut reader));
                let reg = Register::try_from(&mut reader);

                match (dir_ind_reg_a, dir_ind_reg_b, reg) {
                    (Ok(dir_ind_reg_a), Ok(dir_ind_reg_b), Ok(reg)) => Or(dir_ind_reg_a, dir_ind_reg_b, reg),
                    _ => NoOp,
                }
            },
            8 => {
                let first_type = try_param_type!(First, reader);
                let second_type = try_param_type!(Second, reader);

                let dir_ind_reg_a = DirIndReg::try_from((first_type, &mut reader));
                let dir_ind_reg_b = DirIndReg::try_from((second_type, &mut reader));
                let reg = Register::try_from(&mut reader);

                match (dir_ind_reg_a, dir_ind_reg_b, reg) {
                    (Ok(dir_ind_reg_a), Ok(dir_ind_reg_b), Ok(reg)) => Xor(dir_ind_reg_a, dir_ind_reg_b, reg),
                    _ => NoOp,
                }
            },
            9 => ZJump(Direct::from(&mut reader)),
            10 => {
                let first_type = try_param_type!(First, reader);
                let second_type = try_param_type!(Second, reader);

                let dir_ind_reg = DirIndReg::try_from((first_type, &mut reader));
                let dir_reg = DirReg::try_from((second_type, &mut reader));
                let reg = Register::try_from(&mut reader);

                match (dir_ind_reg, dir_reg, reg) {
                    (Ok(dir_ind_reg), Ok(dir_reg), Ok(reg)) => LoadIndex(dir_ind_reg, dir_reg, reg),
                    _ => NoOp,
                }
            },
            11 => {
                let second_type = try_param_type!(Second, reader);
                let third_type = try_param_type!(Third, reader);

                let reg = Register::try_from(&mut reader);
                let dir_ind_reg = DirIndReg::try_from((second_type, &mut reader));
                let dir_reg = DirReg::try_from((third_type, &mut reader));

                match (reg, dir_ind_reg, dir_reg) {
                    (Ok(reg), Ok(dir_ind_reg), Ok(dir_reg)) => StoreIndex(reg, dir_ind_reg, dir_reg),
                    _ => NoOp,
                }
            },
            12 => Fork(Direct::from(&mut reader)),
            13 => {
                let first_type = try_param_type!(First, reader);
                match (DirInd::try_from((first_type, &mut reader)), Register::try_from(&mut reader)) {
                    (Ok(dir_ind), Ok(reg)) => LongLoad(dir_ind, reg),
                    _ => NoOp,
                }
            },
            14 => {
                let first_type = try_param_type!(First, reader);
                let second_type = try_param_type!(Second, reader);

                let dir_ind_reg = DirIndReg::try_from((first_type, &mut reader));
                let dir_reg = DirReg::try_from((second_type, &mut reader));
                let reg = Register::try_from(&mut reader);

                match (dir_ind_reg, dir_reg, reg) {
                    (Ok(dir_ind_reg), Ok(dir_reg), Ok(reg)) => LongLoadIndex(dir_ind_reg, dir_reg, reg),
                    _ => NoOp,
                }
            },
            15 => Longfork(Direct::from(&mut reader)),
            16 => match Register::try_from(&mut reader) {
                Ok(reg) => Display(reg),
                _ => NoOp,
            },
            _ => NoOp,
        }
    }
}
