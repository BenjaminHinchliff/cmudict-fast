//! The pronunciation dictionary from Carnegie Mellon University's CMUSphinx project
#![deny(missing_docs)]
extern crate cmudict_core;
extern crate indexed_line_reader;
extern crate radix_trie;
#[macro_use] extern crate log;

use std::str::FromStr;
use std::sync::Mutex;
use std::cell::RefCell;
use std::io::{BufReader, BufRead, Seek, SeekFrom};
use std::fs::{OpenOptions, File};
use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

use radix_trie::Trie;
use indexed_line_reader::IndexedLineReader;

pub use cmudict_core::{Rule, Stress, Symbol};

pub use errors::*;

mod errors;

type Index = Mutex<RefCell<IndexedLineReader<BufReader<File>>>>;

/// A dictionary containing words & their pronunciations
#[derive(Debug)]
pub struct Cmudict {
    index: Trie<String, (usize, usize)>,
    fname: PathBuf,
    line_index: Index,
}

impl Cmudict {
    /// Takes a path to a cmudict file and tries to construct a `Cmudict` struct
    ///
    /// # Example
    ///
    /// ```
    /// extern crate cmudict;
    /// use cmudict::Cmudict;
    /// # use cmudict::Result;
    /// #
    /// # fn main() {
    /// #   if let Err(_) = run() {
    /// #     panic!("error!");
    /// #   }
    /// # }
    /// # fn run() -> Result<()> {
    ///
    /// let dict = Cmudict::new("./resources/cmudict.dict")?;
    ///
    /// #   Ok(())
    /// # }
    /// ```
    pub fn new<P: AsRef<Path>>(dict: P) -> Result<Cmudict> {
        let path = dict.as_ref();
        let index = make_index(&path)?;
        let file = OpenOptions::new().read(true).open(&path)?;
        let line_index = Mutex::new(RefCell::new(IndexedLineReader::new(BufReader::new(file), 100)));
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
        println!("{:?}", idx);
        idx
    }

    /// Look for a word in the dictionary, and retrieve it's pronunciation
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate cmudict;
    ///
    /// use cmudict::{Cmudict, Symbol, Stress};
    /// # use cmudict::Result;
    /// # 
    /// # fn main() {
    /// #   if let Err(_) = run() {
    /// #     panic!("error");
    /// #   }
    /// # }
    /// # fn run() -> Result<()> {
    ///
    /// let dict = Cmudict::new("./resources/cmudict.dict")?;
    /// let rust = dict.get("rust");
    ///
    /// assert!(rust.is_some());
    /// assert_eq!(
    ///     rust.unwrap().pronunciation(),
    ///     &[Symbol::R,
    ///       Symbol::AH(Stress::Primary),
    ///       Symbol::S,
    ///       Symbol::T]
    /// );
    /// #   Ok(())
    /// # }
    /// ```
    pub fn get(&self, s: &str) -> Option<Rule> {
        self.get_index_val(s).and_then(|(start, end)| {
            let mut lineno = start as u64;
            loop {
                let line = {
                    let lock = self.line_index.lock();
                    match lock {
                        Ok(lock) => {
                            let mut reader = lock.borrow_mut();
                            match reader.seek(SeekFrom::Start(lineno)) {
                                Ok(l) if l == end as u64 => break,
                                Err(e) => {
                                    error!("error while seeking: {:?}", e);
                                    break;
                                },
                                Ok(_) => {},
                            }
                            let mut line = String::new();
                            if reader.read_line(&mut line).is_err() {
                                break
                            }
                            line
                        },
                        Err(e) => {
                            error!("error while locking: {:?}", e);
                            return None;
                        }
                    }
                };
                println!("{}", line);
                let word = if let Some(word) = left(&line) {
                    word
                } else {
                    break
                };
                if word == s {
                    match Rule::from_str(&line) {
                        Ok(rule) => return Some(rule),
                        Err(e) => {
                            error!("error creating rule: {:?}", e);
                            break
                        },
                    }
                } else {
                    lineno += 1;
                }
            }
            None
        })
    }
}

/* Helper functions */

// splits a line on the first space character, and returns the left side
fn left(s: &str) -> Option<&str> {
    let mut parts = s.splitn(2, ' ');
    parts.next()
}

fn make_index<P: AsRef<Path>>(file: P) -> Result<Trie<String, (usize, usize)>> {
    let file = OpenOptions::new().read(true).open(&file)?;
    let reader = BufReader::new(file);
    let mut seen = HashSet::new();
    let mut map = Trie::new();
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
    use std::thread;
    use std::sync::Arc;
    use cmudict_core::{Rule, Symbol, Stress};

    #[test]
    fn test_basics() {
        let d = Cmudict::new("./resources/cmudict.dict").expect("Could not create Cmudict");
        let apple = d.get("apple");
        assert!(apple.is_some());
        assert_eq!(apple,
                Some(Rule::new(
                    "apple".to_string(),
                    vec![
                        Symbol::AE(Stress::Primary),
                        Symbol::P,
                        Symbol::AH(Stress::None),
                        Symbol::L,
                    ]
                )));
        let abf = d.get("abf");
        assert!(abf.is_none());

        let unfit = d.get("unfit");
        assert!(unfit.is_some());
        assert_eq!(unfit,
                Some(Rule::new(
                        "unfit".to_string(),
                        vec![
                            Symbol::AH(Stress::None),
                            Symbol::N,
                            Symbol::F,
                            Symbol::IH(Stress::Primary),
                            Symbol::T]
                )));
    } 

    #[test]
    fn threads() {
        let d = Arc::new(Cmudict::new("./resources/cmudict.dict").expect("Could not create Cmudict"));
        let words = [
            "hello",
            "apple",
            "rust",
        ];
        let mut threads = Vec::with_capacity(words.len());
        for i in 0..words.len() {
            let d = d.clone();
            threads.push(thread::spawn(move || {
                let word = words[i];
                let result = d.get(&word);
                assert!(result.is_some());
            }));
        }
        for thread in threads.into_iter() {
            thread.join().unwrap();
        }
    }
}
