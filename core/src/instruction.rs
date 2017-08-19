use std::convert::TryFrom;
use std::io::{self, Read};
use byteorder::{BigEndian, ReadBytesExt};

const OP_CODE_SIZE:     usize = 1;
const PARAM_CODE_SIZE:  usize = 1;

const REG_SIZE:         usize = 1;
const DIR_SIZE:         usize = 4;
const IND_SIZE:         usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParamCode(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Direct(i32);

impl Direct {
    pub fn mem_size(&self) -> usize {
        DIR_SIZE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Indirect(i16);

impl Indirect {
    pub fn mem_size(&self) -> usize {
        IND_SIZE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Register(u8);

impl Register {
    pub fn mem_size(&self) -> usize {
        REG_SIZE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirInd {
    Direct(Direct),
    Indirect(Indirect),
}

impl DirInd {
    pub fn mem_size(&self) -> usize {
        match *self {
            DirInd::Direct(direct) => direct.mem_size(),
            DirInd::Indirect(indirect) => indirect.mem_size(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum IndReg {
    Indirect(Indirect),
    Register(Register),
}

impl IndReg {
    pub fn mem_size(&self) -> usize {
        match *self {
            IndReg::Indirect(indirect) => indirect.mem_size(),
            IndReg::Register(register) => register.mem_size(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirReg {
    Direct(Direct),
    Register(Register),
}

impl DirReg {
    pub fn mem_size(&self) -> usize {
        match *self {
            DirReg::Direct(direct) => direct.mem_size(),
            DirReg::Register(register) => register.mem_size(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirIndReg {
    Direct(Direct),
    Indirect(Indirect),
    Register(Register),
}

impl DirIndReg {
    pub fn mem_size(&self) -> usize {
        match *self {
            DirIndReg::Direct(direct) => direct.mem_size(),
            DirIndReg::Indirect(indirect) => indirect.mem_size(),
            DirIndReg::Register(register) => register.mem_size(),
        }
    }
}

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
    /// The number of bytes this instruction takes.
    pub fn mem_size(&self) -> usize {
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

impl<'a, R: Read> TryFrom<&'a mut R> for Instruction {
    type Error = io::Error;

    fn try_from(buf: &'a mut R) -> Result<Self, Self::Error> {
        Ok(match buf.read_u8()? {
            1 => {
                let dir = buf.read_i32::<BigEndian>()?;
                Live(Direct(dir))
            },
            2 => {
                let param_code = buf.read_u8()?;
                //
                Load(unimplemented!(), unimplemented!())
            },
            3 => Store(unimplemented!(), unimplemented!()),
            4 => Addition(unimplemented!(), unimplemented!(), unimplemented!()),
            5 => Substraction(unimplemented!(), unimplemented!(), unimplemented!()),
            6 => And(unimplemented!(), unimplemented!(), unimplemented!()),
            7 => Or(unimplemented!(), unimplemented!(), unimplemented!()),
            8 => Xor(unimplemented!(), unimplemented!(), unimplemented!()),
            9 => ZJump(unimplemented!()),
            10 => LoadIndex(unimplemented!(), unimplemented!(), unimplemented!()),
            11 => StoreIndex(unimplemented!(), unimplemented!(), unimplemented!()),
            12 => Fork(unimplemented!()),
            13 => LongLoad(unimplemented!(), unimplemented!()),
            14 => LongLoadIndex(unimplemented!(), unimplemented!(), unimplemented!()),
            15 => Longfork(unimplemented!()),
            16 => Display(unimplemented!()),
            _ => NoOp,
        })
    }
}
