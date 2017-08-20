use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};

const REG_SIZE:         usize = 1;
const DIR_SIZE:         usize = 4;
const IND_SIZE:         usize = 2;

mod direct;
mod indirect;
mod register;

mod dir_ind;
mod dir_ind_reg;
mod dir_reg;
mod ind_reg;

pub use self::direct::Direct;
pub use self::indirect::Indirect;
pub use self::register::{Register, InvalidRegister};

pub use self::dir_ind::DirInd;
pub use self::dir_ind_reg::DirIndReg;
pub use self::dir_reg::DirReg;
pub use self::ind_reg::IndReg;

#[derive(Debug, Clone, Copy)]
pub struct InvalidParamCode;

// TODO: make free-construction impossible
// use private field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    Direct,
    Indirect,
    Register,
}

#[derive(Debug)]
pub enum ParamNumber {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Clone, Copy)]
pub struct ParamCode(u8);

impl ParamCode {
    pub fn param_type_of(&self, param: ParamNumber) -> Result<ParamType, InvalidParamCode> {
        let param_type = match param {
            ParamNumber::First => (self.0 & 0b11000000) >> 6,
            ParamNumber::Second => (self.0 & 0b00110000) >> 4,
            ParamNumber::Third => (self.0 & 0b00001100) >> 2,
            ParamNumber::Fourth => (self.0 & 0b00000011) >> 0,
        };
        match param_type {
            DIR_CODE => Ok(ParamType::Direct),
            IND_CODE => Ok(ParamType::Indirect),
            REG_CODE => Ok(ParamType::Register),
            _ => Err(InvalidParamCode)
        }
    }
}

impl<'a, R: Read + 'a> From<&'a mut R> for ParamCode {
    fn from(reader: &'a mut R) -> Self {
        ParamCode(reader.read_u8().unwrap())
    }
}
