extern crate env_logger;
extern crate corewar;

use std::env::args;
use std::fs::File;
use std::{io, process};
use corewar::Machine;
use corewar::champion::Champion;

fn failable_main() -> io::Result<()> {
    let _ = env_logger::init();

    let enum_args = args().skip(1).enumerate();
    let champions: Result<_, io::Error> = enum_args.map(|(id, path)| {
            let mut file = File::open(&path)?;
            Ok((id as i32, Champion::new(&mut file)?))
        }).collect();

    let mut stdout = io::stdout();
    let mut machine = Machine::new(champions?);

    for cycle_info in machine.cycle_execute(&mut stdout).take(10_000) {
        println!("{:#?}", cycle_info);
    }

    println!("{:?}", machine.arena);

    Ok(())
}

fn main() {
    if let Err(err) = failable_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
