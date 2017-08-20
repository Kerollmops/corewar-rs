mod context;

pub use self::context::{Context, Registers};

use instruction::Instruction;

#[derive(Debug)]
pub struct Process {
    id: usize,
    context: Context,
    cycle_since_last_live: usize,
    remaining_cycles: usize,
    instruction: Instruction,
}
