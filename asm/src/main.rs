extern crate env_logger;
extern crate asm;

use std::process;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use asm::compile;

fn status_main() -> i32 {
    let _ = env_logger::init();

    if let Some(path) = args().skip(1).next() {
        let path = Path::new(&path);
        let mut out = Vec::new();
        let mut input = match File::open(path) {
            Err(err) => {
                eprintln!("{}", err);
                return 1
            },
            Ok(file) => file,
        };

        if let Err(err) = compile(&mut input, &mut out) {
            eprintln!("{}", err);
            return 1
        }

        let path = path.with_extension("cor");
        let path = path.file_name().unwrap();

        let res = File::create(path).and_then(|mut f| copy(&mut out.as_slice(), &mut f));

        if let Err(err) = res {
            eprintln!("{}", err);
            return 1
        }
    }
    return 0
}

fn main() {
    process::exit(status_main());
}
