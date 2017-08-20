use std::ops::{Index, IndexMut, AddAssign, Add};
use instruction::parameter::Register;
use arena::ArenaIndex;
use core::{REG_NUMBER, MEM_SIZE};

#[derive(Debug)]
pub struct Context {
    pub pc: ArenaIndex,
    pub carry: bool,
    pub cycle_since_last_live: usize,
    pub registers: Registers,
}

impl Context {
    pub fn clean_fork(&self) -> Context {
        Context {
            pc: self.pc.clone(),
            carry: self.carry,
            cycle_since_last_live: 0,
            registers: self.registers.clone()
        }
    }
}

#[derive(Debug, Clone)]
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
