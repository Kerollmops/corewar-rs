mod context;

pub use self::context::{Context, Registers};
use instruction::Instruction;
use instruction::Error as InstrError;
use arena::Arena;

#[derive(Debug)]
pub struct Process {
    pub context: Context,
    pub remaining_cycles: usize,
    pub instruction: Option<Instruction>,
}

// FIXME: Add logging here !
impl Process {
    pub fn new(context: Context, arena: &Arena) -> Self {
        let mut reader = arena.read_from(context.pc);
        let maybe_instr = match Instruction::read_from(&mut reader) {
            Ok(instruction) => Some(instruction),
            Err(InstrError::Io(error)) => panic!("{}", error),
            Err(_) => None,
        };
        Process {
            context: context,
            remaining_cycles: maybe_instr.map(|instr| instr.cycle_cost()).unwrap_or(1),
            instruction: maybe_instr,
        }
    }
}
