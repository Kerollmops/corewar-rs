use std::io::Read;
use byteorder::ReadBytesExt;

const REG_SIZE: usize = 1;
const DIR_SIZE: usize = 4;
const IND_SIZE: usize = 2;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidParamCode;

// TODO: make free-construction impossible: use a private field
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
            0b_01 => Ok(ParamType::Direct),
            0b_10 => Ok(ParamType::Indirect),
            0b_11 => Ok(ParamType::Register),
            _ => Err(InvalidParamCode)
        }
    }
}

impl<'a, R: Read> From<&'a mut R> for ParamCode {
    fn from(reader: &'a mut R) -> Self {
        ParamCode(reader.read_u8().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod first {
        use super::*;

        #[test]
        fn is_direct() {
            let mut param: &[u8] = &[0b01000000];
            let param = ParamCode::from(&mut param);
            assert_eq!(param.param_type_of(ParamNumber::First).unwrap(), ParamType::Direct);
        }

        #[test]
        fn is_indirect() {
            let mut param: &[u8] = &[0b10000000];
            let param = ParamCode::from(&mut param);
            assert_eq!(param.param_type_of(ParamNumber::First).unwrap(), ParamType::Indirect);
        }

        #[test]
        fn is_register() {
            let mut param: &[u8] = &[0b11000000];
            let param = ParamCode::from(&mut param);
            assert_eq!(param.param_type_of(ParamNumber::First).unwrap(), ParamType::Register);
        }

        #[test]
        fn is_invalid() {
            let mut param: &[u8] = &[0b00000000];
            let param = ParamCode::from(&mut param);
            assert_eq!(param.param_type_of(ParamNumber::First).unwrap_err(), InvalidParamCode);
        }
    }

    mod third {
        use super::*;

        #[test]
        fn is_direct() {
            let mut param: &[u8] = &[0b00000100];
            let param = ParamCode::from(&mut param);
            assert_eq!(param.param_type_of(ParamNumber::Third).unwrap(), ParamType::Direct);
        }

        #[test]
        fn is_indirect() {
            let mut param: &[u8] = &[0b00001000];
            let param = ParamCode::from(&mut param);
            assert_eq!(param.param_type_of(ParamNumber::Third).unwrap(), ParamType::Indirect);
        }

        #[test]
        fn is_register() {
            let mut param: &[u8] = &[0b00001100];
            let param = ParamCode::from(&mut param);
            assert_eq!(param.param_type_of(ParamNumber::Third).unwrap(), ParamType::Register);
        }

        #[test]
        fn is_invalid() {
            let mut param: &[u8] = &[0b00000000];
            let param = ParamCode::from(&mut param);
            assert_eq!(param.param_type_of(ParamNumber::Third).unwrap_err(), InvalidParamCode);
        }
    }
}
