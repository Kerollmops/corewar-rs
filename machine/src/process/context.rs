use std::ops::{Index, IndexMut};
use instruction::parameter::Register;
use arena::ArenaIndex;
use core::REG_NUMBER;

#[derive(Debug)]
pub struct Context {
    pub pc: ArenaIndex,
    pub carry: bool,
    pub cycle_since_last_live: usize,
    pub registers: Registers,
}

impl Context {
    pub fn new(pc: ArenaIndex) -> Self {
        Context {
            pc: pc,
            carry: false,
            cycle_since_last_live: 0,
            registers: Registers::new(),
        }
    }

    pub fn clean_fork(&self) -> Context {
        Context {
            pc: self.pc,
            carry: self.carry,
            cycle_since_last_live: 0,
            registers: self.registers.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Registers {
    inner: [i32; REG_NUMBER]
}

impl Registers {
    pub fn new() -> Self {
        Registers { inner: [0; REG_NUMBER] }
    }
}

impl Index<Register> for Registers {
    type Output = i32;

    fn index(&self, index: Register) -> &Self::Output {
        &self.inner[*index as usize - 1]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.inner[*index as usize - 1]
    }
}
