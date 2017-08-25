extern crate env_logger;
extern crate core;
extern crate machine;

use std::env::args;
use std::fs::File;
use std::io;
use machine::Machine;
use machine::champion::Champion;

fn failable_main() -> io::Result<()> {
    let enum_args = args().skip(1).enumerate();
    let champions: Result<_, io::Error> = enum_args.map(|(id, path)| {
            let mut file = File::open(&path)?;
            Ok((id as i32, Champion::new(&mut file)?))
        }).collect();

    let mut stdout = io::stdout();
    let mut machine = Machine::new(champions?);

    for _ in machine.cycle_execute(&mut stdout).take(1000) {
        // println!("New cycle");
    }

    // println!("{:?}", machine.arena);

    Ok(())
}

fn main() {
    let _ = env_logger::init();

    if let Err(err) = failable_main() {
        eprintln!("{}", err)
    }
}
