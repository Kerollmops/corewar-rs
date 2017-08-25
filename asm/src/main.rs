extern crate asm;

use std::process;
use std::env::args;
use std::fs::File;
use std::path::Path;
use asm::compile;

fn status_main() -> i32 {
    if let Some(path) = args().skip(1).next() {
        let path = Path::new(&path);
        let res = File::open(path).and_then(|input| {
            let tmp = path.with_extension("cor");
            let output = tmp.file_name().unwrap();
            File::create(output).map(|output| (input, output))
        });

        match res {
            Ok((mut input, mut output)) => {
                if let Err(err) = compile(&mut input, &mut output) {
                    eprintln!("{}", err);
                    return 1
                }
            },
            Err(err) => {
                eprintln!("{}", err);
                return 1
            },
        }
    }
    return 0
}

fn main() {
    process::exit(status_main());
}
