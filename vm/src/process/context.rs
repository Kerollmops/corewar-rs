use std::ops::{Index, IndexMut};
use core::REG_NUMBER;

#[derive(Debug)]
pub struct Context {
    program_counter: usize,
    carry: bool,
    registers: Registers,
}

#[derive(Debug)]
pub struct Registers {
    inner: [i32; REG_NUMBER]
}

impl Index<u8> for Registers {
    type Output = i32;

    fn index(&self, index: u8) -> &Self::Output {
        &self.inner[index as usize - 1]
    }
}

impl IndexMut<u8> for Registers {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.inner[index as usize - 1]
    }
}
