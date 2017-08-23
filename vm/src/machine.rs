use std::collections::BTreeMap;
use std::io;
use process::{Process, Context};
use instruction::Instruction;
use instruction::parameter::Register;
use champion::Champion;
use arena::{Arena, ArenaIndex};
use core::MEM_SIZE;

pub struct Machine {
    pub arena: Arena,
    champions: BTreeMap<i32, Champion>,
    processes: Vec<Process>,
    last_living_champion: Option<i32>,
    number_of_lives: usize,
}

impl Machine {
    pub fn new(champions: BTreeMap<i32, Champion>) -> io::Result<Self> {
        let mut arena = Arena::new();
        let mut arena_index = ArenaIndex::zero();
        let mut processes = Vec::with_capacity(champions.len());
        let step = MEM_SIZE / champions.len();

        for (id, &Champion{ ref program, .. }) in champions.iter() {
            let mut writer = arena.write_to(arena_index);
            io::copy(&mut program.as_slice(), &mut writer)?;
            let instr = Instruction::read_from(&mut program.as_slice());

            let mut context = Context::new(arena_index);
            let reg = unsafe { Register::from_raw(1) };
            context.registers[reg] = *id;

            processes.push(Process {
                context: context,
                remaining_cycles: instr.cycle_cost(),
                instruction: instr,
            });
            arena_index = arena_index.advance_by(step);
        }

        Ok(Machine {
            arena: arena,
            champions: champions,
            processes: processes,
            last_living_champion: None,
            number_of_lives: 0,
        })
    }

    pub fn live_champion(&mut self, champion_id: i32) {
        if let Some(champion) = self.champions.get(&champion_id) {
            self.last_living_champion = Some(champion_id);
            self.number_of_lives += 1;
        }
    }

    pub fn new_process(&mut self, context: Context) {
        let mut reader = self.arena.read_from(context.pc);
        let instruction = Instruction::read_from(&mut reader);
        let process = Process {
            context: context,
            remaining_cycles: instruction.cycle_cost(),
            instruction: instruction,
        };
        self.processes.push(process);
    }
}
