use std::io::{self, Read, Write};
use std::convert::TryFrom;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use instruction::parameter::DIR_SIZE;
use instruction::mem_size::MemSize;
use instruction::const_mem_size::ConstMemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Direct(i32);

impl ConstMemSize for Direct {
    const MEM_SIZE: usize = DIR_SIZE;
}

impl GetValue for Direct {
    fn get_value(&self, _vm: &Machine, _context: &Context) -> i32 {
        self.0
    }
}

impl MemSize for Direct {
    fn mem_size(&self) -> usize {
        Direct::MEM_SIZE
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
