use std::io::{self, Read, Write};
use std::fmt;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use instruction::parameter::DIR_SIZE;
use instruction::mem_size::ConstMemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Direct(pub i32);

impl Direct {
    pub fn read_from<R: Read>(reader: &mut R) -> io::Result<Self> {
        let value = reader.read_i32::<BigEndian>()?;
        let direct = Direct(value);
        Ok(direct)
    }
}

impl ConstMemSize for Direct {
    const MEM_SIZE: usize = DIR_SIZE;
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

impl fmt::Display for Direct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}
