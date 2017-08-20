use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};
use instruction::parameter::IND_SIZE;
use instruction::mem_size::MemSize;
use instruction::get_value::GetValue;
use virtual_machine::VirtualMachine;
use process::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Indirect(i16);

impl GetValue for Indirect {
    fn get_value(&self, vm: &VirtualMachine, context: &Context) -> i32 {
        unimplemented!()
    }
}

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
