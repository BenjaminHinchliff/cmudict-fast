extern crate cmudict_core;
extern crate indexed_line_reader;
#[macro_use] extern crate error_chain;

use std::str::FromStr;
use std::cell::RefCell;
use std::io::{BufReader, BufRead, Seek, SeekFrom};
use std::fs::OpenOptions;
use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::collections::{BTreeMap, HashSet};

use indexed_line_reader::IndexedLineReader;
use cmudict_core::Rule;

use errors::*;

mod errors;

#[derive(Debug)]
pub struct Cmudict {
    index: BTreeMap<String, (usize, usize)>,
    fname: PathBuf,
    line_index: RefCell<IndexedLineReader<BufReader<::std::fs::File>>>,
}

impl Cmudict {
    pub fn new<P: AsRef<Path>>(dict: P) -> Result<Cmudict> {
        let path = dict.as_ref();
        let index = make_index(&path)?;
        let file = OpenOptions::new().read(true).open(&path)?;
        let line_index = RefCell::new(IndexedLineReader::new(BufReader::new(file), 100));
        Ok(Cmudict {
            index: index,
            fname: path.into(),
            line_index: line_index,
        })
    }

    fn get_index_val(&self, s: &str) -> Option<(usize, usize)> {
        let idx = if s.len() < 2 {
            self.index.get(&s[..]).map(|u| *u)
        } else {
            self.index.get(&s[..2]).map(|u| *u)
        };
        idx
    }

    pub fn get(&self, s: &str) -> Option<Rule> {
        self.get_index_val(s).and_then(|(start, end)| {
            let mut reader = self.line_index.borrow_mut();
            if let Err(_) = reader.seek(SeekFrom::Start(start as u64)) {
                return None;
            };
            // need to change something, right now a value that doesn't exist means this is going
            // to search all the way to the end
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).is_err() {
                    break
                }
                let word = if let Some(word) = left(&line) {
                    word
                } else {
                    break
                };
                if word == s {
                    match Rule::from_str(&line) {
                        Ok(rule) => return Some(rule),
                        Err(_) => break,
                    }
                } else {
                    match reader.seek(SeekFrom::Current(1)) {
                        Ok(l) if l == end as u64 => break,
                        Err(_) => break,
                        Ok(_) => {},
                    }
                }
            }
            None
        })
    }
}

fn left(s: &str) -> Option<&str> {
    let mut parts = s.splitn(2, ' ');
    parts.next()
}

fn make_index<P: AsRef<Path>>(file: P) -> Result<BTreeMap<String, (usize, usize)>> {
    let file = OpenOptions::new().read(true).open(&file)?;
    let reader = BufReader::new(file);
    let mut seen = HashSet::new();
    let mut map = BTreeMap::new();
    let mut start = None;
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        if line.starts_with(";;") {
            continue;
        }
        let mut it = line.splitn(2, ' ');
        let label = it.next().unwrap_or("parse error".into());
        let label = split_label(label);
        let word = if label.len() < 2 {
            &label[..]
        } else {
            &label[..2]
        };

        if seen.contains(word) {
            continue;
        }

        match start {
            Some(u) => {
                map.insert(word.to_string(), (u, idx));
                seen.insert(word.to_string());
                start = Some(idx);
            },
            None => {
                start = Some(idx);
            },
        }

    }
    Ok(map)
}

fn split_label(s: &str) -> &str {
    let mut parts = s.rsplitn(2, '(');
    let _ = parts.next();
    if let Some(label) = parts.next() {
        label
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cmudict_core::{Rule, Symbol, Stress};

    #[test]
    fn test_basics() {
        let d = Cmudict::new("./resources/cmudict.dict").expect("Could not create Cmudict");
        let abc = d.get("abc");
        assert!(abc.is_some());
        assert_eq!(abc,
                Some(Rule::new(
                    "abc".to_string(),
                    vec![
                        Symbol::EY("EY1".into(), Stress::Primary),
                        Symbol::B("B".into()),
                        Symbol::IY("IY2".into(), Stress::Secondary),
                        Symbol::S("S".into()),
                        Symbol::IY("IY2".into(), Stress::Secondary)
                    ]
                )));
        let abf = d.get("abf");
        assert!(abf.is_none());
    }
}
