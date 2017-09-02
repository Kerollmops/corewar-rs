use std::io::{self, Read, Error, ErrorKind};
use std::{mem, str};
use core::{Header, COREWAR_EXEC_MAGIC};
use program::Program;

#[derive(Debug)]
pub struct Champion {
    pub name: String,
    pub comment: String,
    pub program: Program,
    _private: (),
}

impl Champion {
    pub fn new<R: Read>(reader: &mut R) -> io::Result<Self> {
        let header: Header = unsafe {
            let mut header = [0u8; mem::size_of::<Header>()];
            reader.read_exact(&mut header)?;
            mem::transmute(header)
        };

        if header.magic.to_be() != COREWAR_EXEC_MAGIC {
            return Err(Error::new(ErrorKind::InvalidData, "invalid magic number"))
        }

        fn into_str_nul_trimmed(slice: &[u8]) -> &str {
            let name = unsafe { str::from_utf8_unchecked(&slice) };
            name.trim_right_matches(|c| c == '\0')
        }

        let name = into_str_nul_trimmed(&header.prog_name);
        let comment = into_str_nul_trimmed(&header.comment);

        info!("champion \"{}\": \"{}\" loaded", name, comment);

        Ok(Champion {
            name: name.to_string(),
            comment: comment.to_string(),
            program: Program::new(header.prog_size.to_be() as usize, reader)?,
            _private: (),
        })
    }
}
