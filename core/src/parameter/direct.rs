use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};
use parameter::DIR_SIZE;
use mem_size::MemSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Direct(i32);

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
