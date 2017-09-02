extern crate env_logger;
extern crate asm;

use std::process;
use std::env::args;
use std::fs::File;
use std::io::{self, copy, Read, Error, ErrorKind};
use std::path::Path;
use asm::compile;

fn failable_main() -> io::Result<()> {
    let _ = env_logger::init();

    let mut args = args().skip(1);
    if args.size_hint().0 > 1 {
        return Err(Error::new(ErrorKind::Other, "Too many arguments."))
    }

    let path = args.next().ok_or(Error::new(ErrorKind::Other,
                        "Missing champion.s file to compile."))?;

    let path = Path::new(&path);
    let mut file = File::open(path)?;

    let input = {
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        buf
    };

    compile(&input).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        .and_then(|out| {
            let path = path.with_extension("cor");
            File::create(&path)
                .and_then(|mut f| copy(&mut out.as_slice(), &mut f))
                .map(|_| path)
        }).map(|path| println!("Writing output program to {}", path.display()))
}

fn main() {
    if let Err(err) = failable_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
