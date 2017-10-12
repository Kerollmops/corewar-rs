extern crate compiler;

use std::io::{self, Read};
use std::path::Path;
use std::{fs, ffi};
use compiler::compile;

fn file_content<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(&path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

#[test]
fn compare_compiled_champions() {
    for entry in fs::read_dir("tests/resources").unwrap().map(|e| e.unwrap()) {
        if !entry.file_type().unwrap().is_file() { continue }

        let path = entry.path();
        if path.extension().is_none() { continue }
        if path.extension() != Some(ffi::OsStr::new("s")) { continue }

        let content = file_content(&path).unwrap();
        let content = String::from_utf8(content).unwrap();
        let compiled = compile(&content).expect(&format!("{:?}", entry));

        let target_path = path.with_extension("cor");
        let target_compiled = file_content(&target_path).unwrap();

        let compiled_bytes = compiled.as_slice();
        let target_compiled_bytes = target_compiled.as_slice();

        if compiled_bytes != target_compiled_bytes {
            panic!("wrong compilation with {}", path.display())
        }
    }
}
