use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};
use instruction::parameter::IND_SIZE;
use instruction::mem_size::MemSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Indirect(i16);

impl MemSize for Indirect {
    fn mem_size(&self) -> usize {
        IND_SIZE
    }
}

impl<'a, R: Read> From<&'a mut R> for Indirect {
    fn from(reader: &'a mut R) -> Self {
        Indirect(reader.read_i16::<BigEndian>().unwrap())
    }
}
