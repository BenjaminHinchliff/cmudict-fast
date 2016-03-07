extern crate cmudict_core;

use std::env;
use std::path::Path;
use std::io::{self, BufRead, Write};
use std::fs::File;

pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest = Path::new(&out_dir).join("lib.rs");
    let src = Path::new("resources/cmudict-0.7b");

    let succeed = expand_rules(&src, &dest);
    println!("{:#?}", succeed);
    assert!(succeed.is_ok());

}

fn expand_rules(src: &Path, dst: &Path) -> io::Result<()> {
    let rules = try!(File::open(src));
    let reader = io::BufReader::new(rules);
    let mut out = try!(File::create(dst));
    let lines = reader.lines();

    try!(writeln!(out, r##"static CMUDICT: Vec<Rule> = vec!["##));

    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("Error reading line, {}", e);
                continue;
            }
        };
        if line.starts_with(";;;") {
            continue;
        }
        let parts = line.split_whitespace();

        try!(writeln!(out, "  {}", line));
    }

    try!(writeln!(out, r##"];"##));

    Ok(())
}

