use std::io::{self, Read, Write};
use core::MEM_SIZE;

pub struct Arena {
    memory: [u8; MEM_SIZE],
}

impl Arena {
    pub fn new() -> Self {
        Arena { memory: [0; MEM_SIZE] }
    }

    pub fn read_from(&self, ArenaIndex(index): ArenaIndex) -> ArenaReader {
        ArenaReader { index, arena: self }
    }

    pub fn write_to(&mut self, ArenaIndex(index): ArenaIndex) -> ArenaWriter {
        ArenaWriter { index, arena: self }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.memory
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArenaIndex(usize);

impl ArenaIndex {
    pub fn from_raw(index: usize) -> Self {
        ArenaIndex::zero().advance_by(index)
    }

    pub fn zero() -> Self {
        ArenaIndex(0)
    }

    pub fn advance_by(self, value: usize) -> Self {
        ArenaIndex((self.0 + value) % MEM_SIZE)
    }

    pub fn move_by(self, value: isize) -> Self {
        let value = if value < 0 {
            MEM_SIZE - ((value % MEM_SIZE as isize) as usize)
        } else {
            value as usize
        };
        self.advance_by(value)
    }
}

pub struct ArenaReader<'a> {
    index: usize,
    arena: &'a Arena,
}

// TODO: use memcpy
impl<'a> Read for ArenaReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut buf_index = 0;
        while buf_index != buf.len() {
            buf[buf_index] = self.arena.memory[self.index];
            buf_index += 1;
            self.index += 1;
            if self.index == self.arena.memory.len() {
                self.index = 0;
            }
        }
        Ok(buf.len())
    }
}

pub struct ArenaWriter<'a> {
    index: usize,
    arena: &'a mut Arena,
}

// TODO: use memcpy
impl<'a> Write for ArenaWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut buf_index = 0;
        while buf_index != buf.len() {
            self.arena.memory[self.index] = buf[buf_index];
            buf_index += 1;
            self.index += 1;
            if self.index == self.arena.memory.len() {
                self.index = 0;
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::MEM_SIZE;

    #[test]
    fn write_read_at_zero() {
        let mut arena = Arena::new();
        let index = ArenaIndex::from_raw(0);

        {
            let mut writer = arena.write_to(index);
            let written = writer.write(&[42, 43, 44]).unwrap();
            assert_eq!(written, 3);
        }

        {
            let mut reader = arena.read_from(index);
            let mut buf = [4, 4, 4];
            let read = reader.read(&mut buf).unwrap();
            assert_eq!(read, 3);
            assert_eq!(&[42, 43, 44], &buf);
        }
    }

    #[test]
    fn write_read_at_limit() {
        let mut arena = Arena::new();
        let index = ArenaIndex::from_raw(MEM_SIZE - 2);

        {
            let mut writer = arena.write_to(index);
            let written = writer.write(&[42, 43, 44]).unwrap();
            assert_eq!(written, 3);
        }

        {
            let mut reader = arena.read_from(index);
            let mut buf = [4, 4, 4];
            let read = reader.read(&mut buf).unwrap();
            assert_eq!(read, 3);
            assert_eq!(&[42, 43, 44], &buf);
        }
    }
}
