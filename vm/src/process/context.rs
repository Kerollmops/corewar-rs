use std::ops::{Index, IndexMut, AddAssign};
use instruction::parameter::Register;
use core::{REG_NUMBER, MEM_SIZE};

#[derive(Debug)]
pub struct Context {
    pub pc: ProgramCounter,
    pub carry: bool,
    pub cycle_since_last_live: usize,
    pub registers: Registers,
}

#[derive(Debug)]
pub struct ProgramCounter {
    inner: usize,
}

// TODO: don't use modulo !
impl AddAssign<isize> for ProgramCounter {
    fn add_assign(&mut self, mut rhs: isize) {
        if rhs < 0 {
            rhs = (rhs % MEM_SIZE as isize) + MEM_SIZE as isize;
        }
        self.inner = ((self.inner as isize + rhs) as usize) % MEM_SIZE;
    }
}

impl AddAssign<usize> for ProgramCounter {
    fn add_assign(&mut self, rhs: usize) {
        self.inner = (self.inner + rhs) % MEM_SIZE;
    }
}

#[derive(Debug)]
pub struct Registers {
    inner: [i32; REG_NUMBER]
}

impl Index<Register> for Registers {
    type Output = i32;

    fn index(&self, index: Register) -> &Self::Output {
        let index: u8 = index.into();
        &self.inner[index as usize - 1]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        let index: u8 = index.into();
        &mut self.inner[index as usize - 1]
    }
}
