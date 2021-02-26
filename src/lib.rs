//! The pronunciation dictionary from Carnegie Mellon University's CMUSphinx project
#![deny(missing_docs)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::{collections::HashMap};

mod core;
mod errors;

pub use crate::core::{Rule, Stress, Symbol};
pub use errors::{Error, ParseError, ParseResult, Result};

/// A dictionary containing words & their pronunciations
#[derive(Debug)]
pub struct Cmudict {
    index: HashMap<String, Vec<Rule>>,
}

impl Cmudict {
    /// Takes a path to a cmudict file and tries to construct a `Cmudict` struct
    ///
    /// # Example
    ///
    /// ```
    /// use cmudict_fast as cmudict;
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
        let index = make_mapping(&path)?;
        Ok(Cmudict { index: index })
    }

    /// Look for a word in the dictionary, and retrieve it's pronunciation
    ///
    /// # Example
    ///
    /// ```rust
    /// use cmudict_fast as cmudict;
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
    ///     rust.unwrap().first().unwrap().pronunciation(),
    ///     &[Symbol::R,
    ///       Symbol::AH(Stress::Primary),
    ///       Symbol::S,
    ///       Symbol::T]
    /// );
    /// #   Ok(())
    /// # }
    /// ```
    pub fn get(&self, s: &str) -> Option<&[Rule]> {
        self.index.get(s).map(|r| &r[..])
    }
}

/// tests whether two words rhyme
pub fn rhymes(ones: &[Rule], twos: &[Rule]) -> bool {
    for one in ones {
        for two in twos {
            let one = one.pronunciation();
            let two = two.pronunciation();
            if let (Some(left), Some(right)) = (
                one.iter().rposition(|s| s.is_syllable()),
                two.iter().rposition(|s| s.is_syllable()),
            ) {
                let one = &one[left..];
                let two = &two[right..];
                if one == two {
                    return true;
                }
            }
        }
    }
    false
}

/* Helper functions */

// splits a line on the hashtag (coment) character, and returns the left side
fn left(s: &str) -> &str {
    let mut parts = s.splitn(2, '#');
    parts.next().unwrap()
}

fn make_mapping<P: AsRef<Path>>(file: P) -> Result<HashMap<String, Vec<Rule>>> {
    let file = File::open(&file)?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        if line.starts_with(";;") {
            continue;
        }
        let label = line
            .splitn(2, ' ')
            .next()
            .ok_or_else(|| Error::InvalidLine(idx))?;
        let label = split_label(label).to_string();
        let rule = Rule::from_str(left(&line))?;
        let rules = map.entry(label).or_insert_with(|| Vec::new());
        rules.push(rule);
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
    use super::core::{Rule, Stress, Symbol};
    use super::*;
    use std::sync::Arc;
    use std::thread;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_basics() {
        let d = Cmudict::new("./resources/cmudict.dict").expect("Could not create Cmudict");
        let apple = d.get("apple");
        assert!(apple.is_some());
        let apple = apple.unwrap().first().unwrap();
        assert_eq!(
            apple,
            &Rule::new(
                "apple".to_string(),
                vec![
                    Symbol::AE(Stress::Primary),
                    Symbol::P,
                    Symbol::AH(Stress::None),
                    Symbol::L,
                ]
            )
        );
        let abf = d.get("abf");
        assert!(abf.is_none());

        let unfit = d.get("unfit");
        assert!(unfit.is_some());
        let unfit = unfit.unwrap().first().unwrap();
        assert_eq!(
            unfit,
            &Rule::new(
                "unfit".to_string(),
                vec![
                    Symbol::AH(Stress::None),
                    Symbol::N,
                    Symbol::F,
                    Symbol::IH(Stress::Primary),
                    Symbol::T
                ]
            )
        );

        let every = d.get("every").unwrap();
        let result = vec![
            Rule::new("every".to_string(), vec![
                Symbol::EH(Stress::Primary),
                Symbol::V,
                Symbol::ER(Stress::None),
                Symbol::IY(Stress::None)
            ]),
            Rule::new("every(2)".to_string(), vec![
                Symbol::EH(Stress::Primary),
                Symbol::V,
                Symbol::R,
                Symbol::IY(Stress::None),
            ]),
        ];
        assert_eq!(every, &result);
    }

    #[test]
    fn rhyming() {
        let d = Cmudict::new("./resources/cmudict.dict").expect("Could not create Cmudict");

        let elf = d.get("elf").unwrap();
        let shelf = d.get("shelf").unwrap();
        assert!(rhymes(elf, shelf));

        let fish = d.get("fish").unwrap();
        assert!(!rhymes(elf, fish));
    }

    #[test]
    fn threads() {
        let d =
            Arc::new(Cmudict::new("./resources/cmudict.dict").expect("Could not create Cmudict"));
        let words = ["hello", "apple", "rust"];
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
