use std::io::{self, Write};

pub trait WriteTo {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}
