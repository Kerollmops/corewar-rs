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
        unimplemented!();
        if index >= MEM_SIZE {
            ArenaIndex(index)
        } else {
            ArenaIndex(index % MEM_SIZE)
        }
    }

    pub fn zero() -> Self {
        ArenaIndex(0)
    }

    pub fn raw_index(&self) -> usize {
        self.0
    }
}

impl Add<ArenaIndex> for ArenaIndex {
    type Output = ArenaIndex;

    fn add(self, ArenaIndex(rhs): ArenaIndex) -> Self {
        if self.0 + rhs >= MEM_SIZE {
            ArenaIndex(self.0 + rhs)
        } else {
            ArenaIndex((self.0 + rhs) % MEM_SIZE)
        }
    }
}

impl Add<isize> for ArenaIndex {
    type Output = ArenaIndex;

    fn add(self, mut rhs: isize) -> Self {
        if rhs < 0 {
            rhs = (rhs % MEM_SIZE as isize) + MEM_SIZE as isize;
        }
        ArenaIndex(((self.0 as isize + rhs) as usize) % MEM_SIZE)
    }
}

impl Add<usize> for ArenaIndex {
    type Output = ArenaIndex;

    fn add(self, rhs: usize) -> Self {
        self + ArenaIndex(rhs)
    }
}

impl AddAssign for ArenaIndex {
    fn add_assign(&mut self, ArenaIndex(rhs): ArenaIndex) {
        if self.0 + rhs >= MEM_SIZE {
            self.0 += rhs;
        } else {
            self.0 = (self.0 + rhs) % MEM_SIZE;
        }
    }
}

impl AddAssign<usize> for ArenaIndex {
    fn add_assign(&mut self, rhs: usize) {
        *self += ArenaIndex(rhs);
    }
}

impl AddAssign<isize> for ArenaIndex {
    fn add_assign(&mut self, rhs: isize) {
        *self = *self + rhs;
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
