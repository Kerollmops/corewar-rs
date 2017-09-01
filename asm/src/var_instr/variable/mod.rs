mod var_dir_ind;
mod var_dir_ind_reg;
mod var_dir_reg;
mod var_ind_reg;

pub use self::var_dir_ind::VarDirInd;
pub use self::var_dir_ind_reg::VarDirIndReg;
pub use self::var_dir_reg::VarDirReg;
pub use self::var_ind_reg::VarIndReg;

use std::convert::TryFrom;
use std::collections::HashMap;
use pest::Error;
use machine::instruction::mem_size::MemSize;
use machine::instruction::const_mem_size::ConstMemSize;
use machine::instruction::parameter::{Direct, Indirect, Register};
use label::Label;

#[derive(Debug)]
pub enum Variable<T> {
    Complete(T),
    Incomplete(Label),
}

impl<T: ConstMemSize> MemSize for Variable<T> {
    fn mem_size(&self) -> usize {
        T::MEM_SIZE
    }
}

pub trait FromPair: Sized {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LabelNotFound(pub Label);

pub trait AsComplete<T> {
    fn as_complete(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<T, LabelNotFound>;
}

impl FromPair for Variable<Direct> {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError> {
        match pair.as_rule() {
            ::Rule::direct => {
                let pair_value = pair.into_inner().next().expect("number not found");
                let span_value = pair_value.clone().into_span();
                match pair_value.as_rule() {
                    ::Rule::number => {
                        let number = i32::from_str_radix(span_value.clone().as_str(), 10);
                        number.map(|n| Variable::Complete(Direct::from(n)))
                              .map_err(|e| Error::CustomErrorSpan { message: e.to_string(), span: span_value })
                    },
                    ::Rule::hexnumber => {
                        let number = i32::from_str_radix(span_value.clone().as_str(), 16);
                        number.map(|n| Variable::Complete(Direct::from(n)))
                              .map_err(|e| Error::CustomErrorSpan { message: e.to_string(), span: span_value })
                    },
                    ::Rule::label_call => Ok(Variable::Incomplete(Label::from(pair_value))),
                    _ => unreachable!()
                }
            },
            _ => Err(Error::CustomErrorSpan {
                message: format!("expected direct found {:?}", pair.as_rule()),
                span: pair.clone().into_span(),
            }),
        }
    }
}

impl AsComplete<Direct> for Variable<Direct> {
    fn as_complete(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<Direct, LabelNotFound> {
        match *self {
            Variable::Complete(direct) => Ok(direct),
            Variable::Incomplete(ref label) => {
                let label_offset = *label_offsets.get(label).ok_or(LabelNotFound(label.clone()))?;
                let value = label_offset as isize - offset as isize;
                Ok(Direct::from(value as i32))
            },
        }
    }
}

impl FromPair for Variable<Indirect> {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError> {
        match pair.as_rule() {
            ::Rule::indirect => {
                let pair_value = pair.into_inner().next().expect("number not found");
                let span_value = pair_value.clone().into_span();
                match pair_value.as_rule() {
                    ::Rule::number => {
                        let number = i16::from_str_radix(span_value.clone().as_str(), 10);
                        number.map(|n| Variable::Complete(Indirect::from(n)))
                              .map_err(|e| Error::CustomErrorSpan { message: e.to_string(), span: span_value })
                    },
                    ::Rule::hexnumber => {
                        let number = i16::from_str_radix(span_value.clone().as_str(), 16);
                        number.map(|n| Variable::Complete(Indirect::from(n)))
                              .map_err(|e| Error::CustomErrorSpan { message: e.to_string(), span: span_value })
                    },
                    ::Rule::label_call => Ok(Variable::Incomplete(Label::from(pair_value))),
                    _ => unreachable!()
                }
            },
            _ => Err(Error::CustomErrorSpan {
                message: format!("expected indirect found {:?}", pair.as_rule()),
                span: pair.clone().into_span(),
            }),
        }
    }
}

impl AsComplete<Indirect> for Variable<Indirect> {
    fn as_complete(&self, offset: usize, label_offsets: &HashMap<Label, usize>) -> Result<Indirect, LabelNotFound> {
        match *self {
            Variable::Complete(indirect) => Ok(indirect),
            Variable::Incomplete(ref label) => {
                let label_offset = *label_offsets.get(label).ok_or(LabelNotFound(label.clone()))?;
                let value = label_offset as isize - offset as isize;
                Ok(Indirect::from(value as i16))
            },
        }
    }
}

impl FromPair for Register {
    fn from_pair(pair: ::AsmPair) -> Result<Self, ::AsmError> {
        match pair.as_rule() {
            ::Rule::register => {
                let pair_number = pair.into_inner().next().expect("number not found");
                let span_number = pair_number.clone().into_span();
                let number = u8::from_str_radix(span_number.clone().as_str(), 10);
                number.map_err(|e| e.to_string())
                      .and_then(|n| Register::try_from(n).map_err(|e| e.to_string()))
                      .map_err(|message| Error::CustomErrorSpan { message, span: span_number })
            },
            _ => Err(Error::CustomErrorSpan {
                message: format!("expected register found {:?}", pair.as_rule()),
                span: pair.clone().into_span(),
            }),
        }
    }
}
