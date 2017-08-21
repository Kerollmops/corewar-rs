use std::io::{self, Read, Write};
use std::ops::{Add, AddAssign};
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
        ArenaIndex(value).advance_by(value)
    }
}

pub struct ArenaReader<'a> {
    index: usize,
    arena: &'a Arena,
}

impl<'a> Read for ArenaReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let memory = self.arena.memory;
        unimplemented!()
    }
}

pub struct ArenaWriter<'a> {
    index: usize,
    arena: &'a mut Arena,
}

impl<'a> Write for ArenaWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
