use std::collections::HashMap;
use process::{Process, Context};
use instruction::Instruction;
use player::Player;
use arena::Arena;

// #[derive(Debug)]
pub struct Machine {
    pub arena: Arena,
    players: HashMap<i32, Player>,
    processes: Vec<Process>,
    last_living_player: Option<i32>,
    number_of_lives: usize,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            arena: Arena::new(),
            players: HashMap::new(),
            processes: Vec::new(),
            last_living_player: None,
            number_of_lives: 0,
        }
    }

    pub fn set_last_living_player(&mut self, player_id: i32) -> Option<&Player> {
        match self.players.get(&player_id) {
            Some(player) => {
                self.last_living_player = Some(player_id);
                self.number_of_lives += 1;
                Some(player)
            },
            None => None,
        }
    }

    pub fn new_process(&mut self, context: Context) {
        unimplemented!();
        // let mut reader = self.arena.read_from(context.pc.raw_value());
        // let instruction = Instruction::from(&mut reader);
        // let process = Process {
        //     context: context,
        //     remaining_cycles: instruction.cycle_cost(),
        //     instruction: instruction,
        // };
        // self.processes.push(process);
    }
}
