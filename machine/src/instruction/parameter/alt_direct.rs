use std::io::{self, Read, Write};
use std::fmt;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use instruction::parameter::ALT_DIR_SIZE;
use instruction::mem_size::ConstMemSize;
use instruction::write_to::WriteTo;
use instruction::get_value::GetValue;
use machine::Machine;
use process::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AltDirect(pub i16);

impl AltDirect {
    pub fn read_from<R: Read>(reader: &mut R) -> io::Result<Self> {
        let value = reader.read_i16::<BigEndian>()?;
        let alt_direct = AltDirect(value);
        Ok(alt_direct)
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

impl fmt::Display for AltDirect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}
