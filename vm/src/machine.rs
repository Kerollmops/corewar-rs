use arena::Arena;
use process::Process;

// #[derive(Debug)]
pub struct Machine {
    arena: Arena,
    processes: Vec<Process>,

    last_living_player: Option<usize>,
    number_of_lives: usize,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            arena: Arena::new(),
            processes: Vec::new(),

            last_living_player: None,
            number_of_lives: 0,
        }
    }

    pub fn set_last_living_player(&mut self, player_id: usize) {
        self.last_living_player = Some(player_id);
        self.number_of_lives += 1;
    }
}
