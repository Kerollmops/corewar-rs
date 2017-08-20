use std::io::{self, Read, Write};
use std::ops::Add;
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
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArenaIndex(usize);

impl ArenaIndex {
    pub fn from_raw(index: usize) -> Self {
        unimplemented!();
        if index >= MEM_SIZE {
            ArenaIndex(index)
        } else {
            ArenaIndex(index % MEM_SIZE)
        }
    }

    pub fn raw_index(&self) -> usize {
        self.0
    }
}

impl Add for ArenaIndex {
    type Output = ArenaIndex;

    fn add(self, ArenaIndex(rhs): ArenaIndex) -> Self {
        if self.0 + rhs >= MEM_SIZE {
            ArenaIndex(self.0 + rhs)
        } else {
            ArenaIndex((self.0 + rhs) % MEM_SIZE)
        }
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
