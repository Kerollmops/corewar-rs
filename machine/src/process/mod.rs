mod context;

pub use self::context::{Context, Registers};
use instruction::Instruction;
use instruction::Error as InstrError;
use arena::Arena;

#[derive(Debug)]
pub struct Process {
    pub context: Context,
    pub remaining_cycles: usize,
    pub instruction: Instruction,
}

// FIXME: Add logging here !
impl Process {
    pub fn new(context: Context, arena: &Arena) -> Self {
        let mut reader = arena.read_from(context.pc);
        let instruction = match Instruction::read_from(&mut reader) {
            Ok(instruction) => instruction,
            Err(InstrError::InvalidCode(_)) => Instruction::NoOp,
            Err(InstrError::InvalidParamCode(_)) => Instruction::NoOp,
            Err(InstrError::InvalidParam) => Instruction::NoOp,
            Err(InstrError::Io(error)) => panic!("{}", error),
        };
        Process {
            context: context,
            remaining_cycles: instruction.cycle_cost(),
            instruction: instruction,
        }
    }
}
