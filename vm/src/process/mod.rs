mod context;

pub use self::context::{Context, Registers};

use instruction::Instruction;

#[derive(Debug)]
pub struct Process {
    pub context: Context,
    pub remaining_cycles: usize,
    pub instruction: Instruction,
}
