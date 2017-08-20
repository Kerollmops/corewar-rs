use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use instruction::parameter::IND_SIZE;
use instruction::mem_size::MemSize;
use instruction::get_value::GetValue;
use instruction::set_value::SetValue;
use machine::Machine;
use process::Context;
use core::IDX_MOD;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Indirect(i16);

impl GetValue for Indirect {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32 {
        let addr = context.pc + (self.0 as isize % IDX_MOD as isize);
        let mut reader = vm.arena.read_from(addr);
        reader.read_i32::<BigEndian>().unwrap()
    }

    fn get_value_long(&self, vm: &Machine, context: &Context) -> i32 {
        let addr = context.pc + self.0 as isize;
        let mut reader = vm.arena.read_from(addr);
        reader.read_i32::<BigEndian>().unwrap()
    }
}

impl SetValue for Indirect {
    fn set_value(&self, value: i32, vm: &mut Machine, context: &Context) {
        let addr = context.pc + (self.0 as isize % IDX_MOD as isize);
        let mut writer = vm.arena.write_to(addr);
        writer.write_i32::<BigEndian>(value).unwrap();
    }

    fn set_value_long(&self, value: i32, vm: &mut Machine, context: &Context) {
        let addr = context.pc + self.0 as isize;
        let mut writer = vm.arena.write_to(addr);
        writer.write_i32::<BigEndian>(value).unwrap();
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

impl From<i16> for Indirect {
    fn from(value: i16) -> Self {
        Indirect(value)
    }
}
