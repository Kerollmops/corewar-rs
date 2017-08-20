mod context;

pub use self::context::{Context, Registers};

use instruction::Instruction;

#[derive(Debug)]
pub struct Process {
    id: usize,
    context: Context,
    remaining_cycles: usize,
    instruction: Instruction,
}
