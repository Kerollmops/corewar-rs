use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io::{self, Write};
use std::mem;
use process::{Process, Context};
use instruction::parameter::Register;
use instruction::Instruction;
use champion::Champion;
use arena::{Arena, ArenaIndex};
use core::{MEM_SIZE, CYCLE_TO_DIE, CYCLE_DELTA, NBR_LIVE, MAX_CHECKS};

pub struct Machine {
    pub arena: Arena,
    champions: BTreeMap<i32, Champion>,
    processes: Vec<Process>,
    last_living_champion: Option<i32>,

    number_of_lives: usize,
    cycles_to_die: usize,
    cycles: usize,
    cycle_checks: usize,
}

impl Machine {
    pub fn new(champions: BTreeMap<i32, Champion>) -> Self {
        let mut arena = Arena::new();
        let mut arena_index = ArenaIndex::zero();
        let mut processes = Vec::with_capacity(champions.len());
        let step = MEM_SIZE.checked_div(champions.len()).unwrap_or(0);

        for (id, &Champion{ ref program, .. }) in champions.iter() {
            {
                let mut writer = arena.write_to(arena_index);
                io::copy(&mut program.as_slice(), &mut writer).unwrap();
            }

            let mut context = Context::new(arena_index);
            let reg = Register::try_from(1).unwrap();
            context.registers[reg] = *id;

            processes.push(Process::new(context, &arena));

            arena_index = arena_index.advance_by(step);
        }

        Machine {
            arena: arena,
            champions: champions,
            processes: processes,
            last_living_champion: None,
            number_of_lives: 0,
            cycles_to_die: CYCLE_TO_DIE,
            cycles: 0,
            cycle_checks: 0,
        }
    }

    pub fn live_champion(&mut self, champion_id: i32) {
        if self.champions.get(&champion_id).is_some() {
            self.last_living_champion = Some(champion_id);
            self.number_of_lives += 1;
        }
    }

    pub fn new_process(&mut self, context: Context) {
        self.processes.push(Process::new(context, &self.arena))
    }

    pub fn cycle_execute<'a, W: Write>(&'a mut self, output: &'a mut W) -> CycleExecute<'a, W> {
        CycleExecute { machine: self, output }
    }
}

pub struct CycleExecute<'a, W: 'a + Write> {
    machine: &'a mut Machine,
    output: &'a mut W,
}

impl<'a, W: 'a + Write> Iterator for CycleExecute<'a, W> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let mut processes = Vec::new();
        mem::swap(&mut processes, &mut self.machine.processes);

        self.machine.cycles += 1;
        if self.machine.cycles >= self.machine.cycles_to_die {
            self.machine.cycle_checks += 1;
            processes.retain(|p| p.context.cycle_since_last_live < self.machine.cycles_to_die);
            if self.machine.number_of_lives >= NBR_LIVE || self.machine.cycle_checks >= MAX_CHECKS {
                self.machine.cycles_to_die -= CYCLE_DELTA;
                self.machine.cycle_checks = 0;
            }
            self.machine.cycles = 0;
            self.machine.number_of_lives = 0;
        }

        for process in &mut processes {
            let ref mut ctx = process.context;
            process.remaining_cycles -= 1;
            ctx.cycle_since_last_live += 1;

            if process.remaining_cycles == 0 {
                let ref mut instr = process.instruction;

                let from = ctx.pc; // TODO: remove

                instr.execute(&mut self.machine, ctx, &mut self.output);

                let to = ctx.pc; // TODO: remove

                trace!("execute {:?}", instr);
                trace!("move from {:?} to {:?}", from, to);

                let reader = self.machine.arena.read_from(ctx.pc);
                *instr = Instruction::read_from(reader);
                process.remaining_cycles = instr.cycle_cost();
            }
        }
        self.machine.processes.append(&mut processes);
        Some(())
    }
}
