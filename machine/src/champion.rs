use std::io::{self, Read, Error, ErrorKind};
use byteorder::{ReadBytesExt, BigEndian};
use core::{COREWAR_EXEC_MAGIC, PROG_NAME_LENGTH, COMMENT_LENGTH};
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
        let magic = reader.read_u32::<BigEndian>()?;
        if magic != COREWAR_EXEC_MAGIC {
            return Err(Error::new(ErrorKind::InvalidData, "invalid magic number"))
        }

        let mut program_name = [0; PROG_NAME_LENGTH + 1];
        let program_name = {
            reader.read_exact(&mut program_name[..PROG_NAME_LENGTH])?;
            let first_nul = program_name.iter().position(|x| *x == 0).unwrap();
            &program_name[..first_nul]
        };

        let program_size = reader.read_u32::<BigEndian>()? as usize;

        let mut comment = [0; COMMENT_LENGTH + 1];
        let comment = {
            reader.read_exact(&mut comment[..COMMENT_LENGTH])?;
            let first_nul = comment.iter().position(|x| *x == 0).unwrap();
            &comment[..first_nul]
        };

        Ok(Champion {
            name: String::from_utf8_lossy(program_name).into(),
            comment: String::from_utf8_lossy(comment).into(),
            program: Program::new(program_size, reader)?,
            _private: (),
        })
    }
}
