extern crate env_logger;
extern crate corewar;

use std::env::args;
use std::fs::File;
use std::net::TcpStream;
use std::{io, process};
use corewar::Machine;
use corewar::champion::Champion;

fn failable_main() -> io::Result<()> {
    let _ = env_logger::init();

    let enum_args = args().skip(1).enumerate();
    let champions: Result<_, io::Error> = enum_args.map(|(id, path)| {
            let mut file = File::open(&path)?;
            println!("reading file at {}", path);
            Ok((id as i32, Champion::new(&mut file)?))
        }).collect();

    let mut talk_stream = TcpStream::connect("127.0.0.1:14315").map(|x| Box::new(x) as Box<io::Write>)
                                .unwrap_or_else(|_| Box::new(io::sink()));

    let mut machine = Machine::new(champions?);

    for cycle_info in machine.cycle_execute(&mut talk_stream) {
        println!("{:#?}", cycle_info);
    }

    println!("{:?}", machine.arena);

    match machine.last_living_champion() {
        Some((id, champ)) => println!("A winner is {}({}), {}", id, champ.name, champ.comment),
        None => println!("Sadly, no winner has been found"),
    }

    Ok(())
}

fn main() {
    if let Err(err) = failable_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
