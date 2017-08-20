use arena::Arena;
use process::Process;

// #[derive(Debug)]
pub struct VirtualMachine {
    arena: Arena,
    processes: Vec<Process>
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            arena: Arena::new(),
            processes: Vec::new(),
        }
    }

    //
}
