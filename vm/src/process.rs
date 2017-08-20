use core::REG_NUMBER;
use instruction::Instruction;

#[derive(Debug)]
pub struct Process {
    id: usize,

    program_counter: usize,
    carry: bool,
    registers: [i32; REG_NUMBER], // be carreful, starts at one

    cycle_since_last_live: usize,
    remaining_cycles: usize,

    instruction: Instruction,
}

//
