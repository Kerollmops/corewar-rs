use std::io::Write;

pub trait WriteTo {
    fn write_to<W: Write>(&self, writer: &mut W);
}
