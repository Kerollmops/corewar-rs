use std::io::{self, Read, Write};
use std::fmt;
use std::convert::TryFrom;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use instruction::parameter::IND_SIZE;
use instruction::mem_size::MemSize;
use instruction::const_mem_size::ConstMemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use instruction::set_value::SetValue;
use machine::Machine;
use process::Context;
use core::IDX_MOD;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Indirect(i16);

impl Indirect {
    pub fn value(&self) -> i16 {
        self.0
    }
}

impl ConstMemSize for Indirect {
    const MEM_SIZE: usize = IND_SIZE;
}

impl GetValue for Indirect {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32 {
        let addr = context.pc.move_by(self.0 as isize % IDX_MOD as isize);
        let mut reader = vm.arena.read_from(addr);
        reader.read_i32::<BigEndian>().unwrap()
    }

    fn get_value_long(&self, vm: &Machine, context: &Context) -> i32 {
        let addr = context.pc.move_by(self.0 as isize);
        let mut reader = vm.arena.read_from(addr);
        reader.read_i32::<BigEndian>().unwrap()
    }
}

impl SetValue for Indirect {
    fn set_value(&self, value: i32, vm: &mut Machine, context: &Context) {
        let addr = context.pc.move_by(self.0 as isize % IDX_MOD as isize);
        let mut writer = vm.arena.write_to(addr);
        writer.write_i32::<BigEndian>(value).unwrap();
    }

    fn set_value_long(&self, value: i32, vm: &mut Machine, context: &Context) {
        let addr = context.pc.move_by(self.0 as isize);
        let mut writer = vm.arena.write_to(addr);
        writer.write_i32::<BigEndian>(value).unwrap();
    }
}

impl MemSize for Indirect {
    fn mem_size(&self) -> usize {
        Indirect::MEM_SIZE
    }
}

impl WriteTo for Indirect {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i16::<BigEndian>(self.0)
    }
}

impl<'a, R: Read> TryFrom<&'a mut R> for Indirect {
    type Error = io::Error;

    fn try_from(reader: &'a mut R) -> Result<Self, Self::Error> {
        let value = reader.read_i16::<BigEndian>()?;
        Ok(Indirect::from(value))
    }
}

impl From<i16> for Indirect {
    fn from(value: i16) -> Self {
        Indirect(value)
    }
}

impl fmt::Display for Indirect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
