use std::io::{self, Read, Write};
use core::MEM_SIZE;

pub struct Arena {
    memory: [u8; MEM_SIZE],
}

impl Arena {
    pub fn new() -> Self {
        Arena { memory: [0; MEM_SIZE] }
    }

    pub fn read_from(&self, index: usize) -> ArenaReader {
        ArenaReader { index, arena: self }
    }

    pub fn write_to(&mut self, index: usize) -> ArenaWriter {
        unimplemented!()
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
