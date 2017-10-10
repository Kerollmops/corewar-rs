use std::io::{self, Read, Write};
use std::fmt;
use std::convert::TryFrom;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use instruction::parameter::ALT_DIR_SIZE;
use instruction::mem_size::ConstMemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AltDirect(i16);

impl AltDirect {
    pub fn value(&self) -> i16 {
        self.0
    }
}

impl ConstMemSize for AltDirect {
    const MEM_SIZE: usize = ALT_DIR_SIZE;
}

impl GetValue for AltDirect {
    fn get_value(&self, _vm: &Machine, _context: &Context) -> i32 {
        self.0 as i32
    }
}

impl WriteTo for AltDirect {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i16::<BigEndian>(self.0)
    }
}

impl<'a, R: Read> TryFrom<&'a mut R> for AltDirect {
    type Error = io::Error;

    fn try_from(reader: &'a mut R) -> Result<Self, Self::Error> {
        let value = reader.read_i16::<BigEndian>()?;
        Ok(AltDirect::from(value))
    }
}

impl From<i16> for AltDirect {
    fn from(value: i16) -> Self {
        AltDirect(value)
    }
}

impl From<AltDirect> for i16 {
    fn from(alt_direct: AltDirect) -> Self {
        alt_direct.0
    }
}

impl fmt::Display for AltDirect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}
