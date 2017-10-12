use std::io::{self, Read, Write};
use std::fmt;
use std::convert::TryFrom;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use instruction::parameter::{ParamType, ParamTypeOf};
use instruction::parameter::DIR_SIZE;
use instruction::mem_size::ConstMemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Direct(i32);

impl Direct {
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl ConstMemSize for Direct {
    const MEM_SIZE: usize = DIR_SIZE;
}

impl ParamTypeOf for Direct {
    fn param_type(&self) -> ParamType {
        ParamType::Direct
    }
}

impl GetValue for Direct {
    fn get_value(&self, _vm: &Machine, _context: &Context) -> i32 {
        self.0
    }
}

impl WriteTo for Direct {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i32::<BigEndian>(self.0)
    }
}

impl<'a, R: Read> TryFrom<&'a mut R> for Direct {
    type Error = io::Error;

    fn try_from(reader: &'a mut R) -> Result<Self, Self::Error> {
        let value = reader.read_i32::<BigEndian>()?;
        Ok(Direct::from(value))
    }
}

impl From<i32> for Direct {
    fn from(value: i32) -> Self {
        Direct(value)
    }
}

impl From<Direct> for i32 {
    fn from(direct: Direct) -> Self {
        direct.0
    }
}

impl fmt::Display for Direct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}
