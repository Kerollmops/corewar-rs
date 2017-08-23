mod context;

pub use self::context::{Context, Registers};
use instruction::Instruction;
use arena::Arena;

#[derive(Debug)]
pub struct Process {
    pub context: Context,
    pub remaining_cycles: usize,
    pub instruction: Instruction,
}

impl Process {
    pub fn new(context: Context, arena: &Arena) -> Self {
        let mut reader = arena.read_from(context.pc);
        let instruction = Instruction::read_from(&mut reader);
        Process {
            context: context,
            remaining_cycles: instruction.cycle_cost(),
            instruction: instruction,
        }
    }
}
