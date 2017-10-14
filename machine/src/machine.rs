use std::collections::{BTreeMap, HashMap};
use std::io::{self, Write};
use std::mem;
use process::{Process, Context};
use instruction::parameter::{Direct, Register};
use instruction::Instruction;
use instruction::Error as InstrError;
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

        for (id, &Champion{ ref program, .. }) in &champions {
            {
                let mut writer = arena.write_to(arena_index);
                io::copy(&mut program.as_slice(), &mut writer).unwrap();
            }

            let mut context = Context::new(arena_index);
            let reg = Register::new(1).unwrap();
            context.registers[reg] = *id;

            let process = Process::new(context, &arena);
            trace!("push process {:?}", process);
            processes.push(process);

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

    pub fn last_living_champion(&self) -> Option<(i32, &Champion)> {
        self.last_living_champion
            .and_then(|id| self.champions.get(&id)
            .map(|champ| (id, champ)))
    }

    pub fn new_process(&mut self, context: Context) {
        let process = Process::new(context, &self.arena);
        trace!("push process {:?}", process);
        self.processes.push(process)
    }

    pub fn cycles_to_die(&self) -> usize {
        self.cycles_to_die
    }

    pub fn cycle_execute<'a, W: Write>(&'a mut self, output: &'a mut W) -> CycleExecute<'a, W> {
        CycleExecute { machine: self, output }
    }
}

pub struct CycleExecute<'a, W: 'a + Write> {
    machine: &'a mut Machine,
    output: &'a mut W,
}

#[derive(Debug, Clone, Default)]
pub struct CycleInfo {
    pub remaining_processes: usize,
    pub cycles_to_die: usize,
    pub lives_counter: HashMap<i32, usize>,
    pub last_living_champion: Option<i32>,
}

impl<'a, W: 'a + Write> Iterator for CycleExecute<'a, W> {
    type Item = io::Result<CycleInfo>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut processes = Vec::new();
        mem::swap(&mut processes, &mut self.machine.processes);

        let mut cycle_info = CycleInfo {
            lives_counter: HashMap::with_capacity(self.machine.champions.len()),
            ..Default::default()
        };

        self.machine.cycles += 1;
        if self.machine.cycles >= self.machine.cycles_to_die {
            self.machine.cycle_checks += 1;
            processes.retain(|p| p.context.cycle_since_last_live < self.machine.cycles_to_die);
            if self.machine.number_of_lives >= NBR_LIVE || self.machine.cycle_checks >= MAX_CHECKS {
                self.machine.cycles_to_die = self.machine.cycles_to_die.saturating_sub(CYCLE_DELTA);
                self.machine.cycle_checks = 0;
            }
            self.machine.cycles = 0;
            self.machine.number_of_lives = 0;
        }

        cycle_info.cycles_to_die = self.machine.cycles_to_die - self.machine.cycles;

        for process in processes.iter_mut().rev() {
            let ctx = &mut process.context;
            process.remaining_cycles -= 1;
            ctx.cycle_since_last_live += 1;

            if process.remaining_cycles == 0 {
                let instr = &mut process.instruction;
                match *instr {
                    Some(instr) => if let Err(e) = instr.execute(&mut self.machine, ctx, &mut self.output) {
                        return Some(Err(e))
                    },
                    None => Instruction::execute_noop(ctx)
                }
                trace!("execute {:?}", instr);

                if let Some(Instruction::Live(Direct(champion_id))) = *instr {
                    if self.machine.champions.contains_key(&champion_id) {
                        let counter = cycle_info.lives_counter.entry(champion_id).or_insert(0);
                        *counter += 1;
                    }
                }

                let reader = self.machine.arena.read_from(ctx.pc);
                *instr = match Instruction::read_from(reader) {
                    Ok(instr) => Some(instr),
                    Err(InstrError::Io(e)) => return Some(Err(e)),
                    Err(_) => None,
                };
                process.remaining_cycles = instr.map(|instr| instr.cycle_cost()).unwrap_or(1);
            }
        }
        self.machine.processes.append(&mut processes);
        cycle_info.remaining_processes = self.machine.processes.len();
        cycle_info.last_living_champion = self.machine.last_living_champion;

        if !self.machine.processes.is_empty() {
            Some(Ok(cycle_info))
        } else { None }
    }
}
