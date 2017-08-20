use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};
use instruction::parameter::DIR_SIZE;
use instruction::mem_size::MemSize;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Direct(i32);

impl GetValue for Direct {
    fn get_value(&self, _vm: &Machine, _context: &Context) -> i32 {
        self.0
    }
}

impl MemSize for Direct {
    fn mem_size(&self) -> usize {
        DIR_SIZE
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
