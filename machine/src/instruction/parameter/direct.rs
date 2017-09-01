use std::io::{Read, Write};
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
    fn write_to<W: Write>(&self, writer: &mut W) {
        let _ = writer.write_i32::<BigEndian>(self.0);
    }
}

impl<'a, R: Read> From<&'a mut R> for Direct {
    fn from(reader: &'a mut R) -> Self {
        Direct(reader.read_i32::<BigEndian>().unwrap())
    }
}

impl From<Direct> for i32 {
    fn from(direct: Direct) -> Self {
        direct.0
    }
}

impl From<i32> for Direct {
    fn from(value: i32) -> Self {
        Direct(value)
    }
}
