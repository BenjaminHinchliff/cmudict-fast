#[macro_use] extern crate error_chain;

use std::str::FromStr;
use std::fmt;

pub use errors::*;

mod errors {
    error_chain!{
        errors {
            ParseError(t: String) {
                description("parse error")
                display("parse error: {}", t)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stress {
    None,
    Primary,
    Secondary,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    AA(Stress),
    AH(Stress),
    AO(Stress),
    AW(Stress),
    AY(Stress),
    B,
    CH,
    D,
    DH,
    EH(Stress),
    ER(Stress),
    EY(Stress),
    F,
    G,
    HH,
    IH(Stress),
    IY(Stress),
    JH,
    K,
    L,
    M,
    N,
    NG,
    OW(Stress),
    OY(Stress),
    P,
    R,
    S,
    SH,
    T,
    TH,
    UH(Stress),
    UW(Stress),
    V,
    W,
    Y,
    Z,
    ZH,
}

fn parse_error(s: &str) -> Error {
    ErrorKind::ParseError(s.into()).into()
}

fn parse_error_expect(before: &str, after: &str, c: char) -> Error {
    parse_error(&format!("Expected {} after {}, got {}", before, after, c))
}

fn parse_error_eof(before: &str, after: &str) -> Error {
    parse_error(&format!("Expected {} after {}, got EOF", before, after))
}

macro_rules! parse_stress {
    ( $next:expr, $symbol:expr ) => {{
        match $next {
            Some('0') | None => Ok($symbol(Stress::None)),
            Some('1') => Ok($symbol(Stress::Primary)),
            Some('2') => Ok($symbol(Stress::Secondary)),
            Some(c) => Err(parse_error(&format!("Expected stress marker '0', '1', or '2', got {}", c))),
        }
    }}
}

impl fmt::Display for Stress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Stress::None => write!(f, "{}", 0),
            Stress::Primary => write!(f, "{}", 1),
            Stress::Secondary => write!(f, "{}", 2),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Symbol::AA(ref s1) => {
                write!(f, "AA{}", s1)
            },
            Symbol::AH(ref s1) => {
                write!(f, "AH{}", s1)
            },
            Symbol::AO(ref s1) => {
                write!(f, "AO{}", s1)
            },
            Symbol::AW(ref s1) => {
                write!(f, "AW{}", s1)
            },
            Symbol::AY(ref s1) => {
                write!(f, "AY{}", s1)
            },
            Symbol::B => {
                write!(f, "B")
            },
            Symbol::CH => {
                write!(f, "CH")
            },
            Symbol::D => {
                write!(f, "D")
            },
            Symbol::DH => {
                write!(f, "DH")
            },
            Symbol::EH(ref s1) => {
                write!(f, "EH{}", s1)
            },
            Symbol::ER(ref s1) => {
                write!(f, "ER{}", s1)
            },
            Symbol::EY(ref s1) => {
                write!(f, "EY{}", s1)
            },
            Symbol::F => {
                write!(f, "F")
            },
            Symbol::G => {
                write!(f, "G")
            },
            Symbol::HH => {
                write!(f, "HH")
            },
            Symbol::IH(ref s1) => {
                write!(f, "IH{}", s1)
            },
            Symbol::IY(ref s1) => {
                write!(f, "IY{}", s1)
            },
            Symbol::JH => {
                write!(f, "JH")
            },
            Symbol::K => {
                write!(f, "K")
            },
            Symbol::L => {
                write!(f, "L")
            },
            Symbol::M => {
                write!(f, "M")
            },
            Symbol::N => {
                write!(f, "N")
            },
            Symbol::NG => {
                write!(f, "NG")
            },
            Symbol::OW(ref s1) => {
                write!(f, "OW{}", s1)
            },
            Symbol::OY(ref s1) => {
                write!(f, "OY{}", s1)
            },
            Symbol::P => {
                write!(f, "P")
            },
            Symbol::R => {
                write!(f, "R")
            },
            Symbol::S => {
                write!(f, "S")
            },
            Symbol::SH => {
                write!(f, "SH")
            },
            Symbol::T => {
                write!(f, "T")
            },
            Symbol::TH => {
                write!(f, "TH")
            },
            Symbol::UH(ref s1) => {
                write!(f, "UH{}", s1)
            },
            Symbol::UW(ref s1) => {
                write!(f, "UW{}", s1)
            },
            Symbol::V => {
                write!(f, "V")
            },
            Symbol::W => {
                write!(f, "W")
            },
            Symbol::Y => {
                write!(f, "Y")
            },
            Symbol::Z => {
                write!(f, "Z")
            },
            Symbol::ZH => {
                write!(f, "ZH")
            },
        }
    }
}

impl FromStr for Symbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Symbol> {
        let mut chrs = s.chars();

        match chrs.next() {
            None => Err(ErrorKind::ParseError("Expected character, got EOF".into()).into()),
            Some('A') => {
                match chrs.next() {
                    Some('A') => parse_stress!( chrs.next(), Symbol::AA ),
                    Some('H') => parse_stress!( chrs.next(), Symbol::AH ),
                    Some('O') => parse_stress!( chrs.next(), Symbol::AO ),
                    Some('W') => parse_stress!( chrs.next(), Symbol::AW ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::AY ),
                    Some(c) => Err(parse_error_expect("A, H, O, W, or Y", "A", c)),
                    None => Err(parse_error_eof("A, H, O, W, or Y", "A")),
                }
            },
            Some('E') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::EH ),
                    Some('R') => parse_stress!( chrs.next(), Symbol::ER ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::EY ),
                    Some(c) => Err(parse_error_expect("H, R, or Y", "E", c)),
                    None => Err(parse_error_eof("H, R, or Y", "E")),
                }
            },
            Some('I') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::IH ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::IY ),
                    Some(c) => Err(parse_error_expect("H or Y", "I", c)),
                    None => Err(parse_error_eof("H or Y", "I")),
                }
            },
            Some('O') => {
                match chrs.next() {
                    Some('W') => parse_stress!( chrs.next(), Symbol::OW ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::OY ),
                    Some(c) => Err(parse_error_expect("W or Y", "O", c)),
                    None => Err(parse_error_eof("W or Y", "O")),
                }
            },
            Some('U') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::UH ),
                    Some('W') => parse_stress!( chrs.next(), Symbol::UW ),
                    Some(c) => Err(parse_error_expect("H or W", "U", c)),
                    None => Err(parse_error_eof("H or W", "U")),
                }
            },
            Some('B') => Ok(Symbol::B),
            Some('C') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::CH),
                    Some(c) => Err(parse_error_expect("H", "C", c)),
                    None => Err(parse_error_eof("H", "C")),
                }
            },
            Some('D') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::DH),
                    None => Ok(Symbol::D),
                    Some(c) => Err(parse_error_expect("H or EOF", "D", c)),
                }
            },
            Some('F') => Ok(Symbol::F),
            Some('G') => Ok(Symbol::G),
            Some('H') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::HH),
                    Some(c) => Err(parse_error_expect("H", "H", c)),
                    None => Err(parse_error_eof("H", "H")),
                }
            },
            Some('J') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::JH),
                    Some(c) => Err(parse_error_expect("H", "J", c)),
                    None => Err(parse_error_eof("H", "J")),
                }
            },
            Some('K') => Ok(Symbol::K),
            Some('L') => Ok(Symbol::L),
            Some('M') => Ok(Symbol::M),
            Some('N') => {
                match chrs.next() {
                    Some('G') => Ok(Symbol::NG),
                    None => Ok(Symbol::N),
                    Some(c) => Err(parse_error_expect("G or EOF", "N", c)),
                }
            },
            Some('P') => Ok(Symbol::P),
            Some('R') => Ok(Symbol::R),
            Some('S') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::SH),
                    None => Ok(Symbol::S),
                    Some(c) => Err(parse_error_expect("H or EOF", "S", c)),
                }
            },
            Some('T') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::TH),
                    None => Ok(Symbol::T),
                    Some(c) => Err(parse_error_expect("H or EOF", "T", c)),
                }
            },
            Some('V') => Ok(Symbol::V),
            Some('W') => Ok(Symbol::W),
            Some('Y') => Ok(Symbol::Y),
            Some('Z') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::ZH),
                    None => Ok(Symbol::Z),
                    Some(c) => Err(parse_error_expect("H or EOF", "Z", c)),
                }
            },
            Some(c) => Err(parse_error(&format!("Expected A-Z, got {}", c))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    label: String,
    pronunciation: Vec<Symbol>,
}


impl Rule {
    pub fn new(label: String, pronunciation: Vec<Symbol>) -> Rule {
        Rule {
            label: label,
            pronunciation: pronunciation,
        }
    }

    pub fn pronunciation(&self) -> &[Symbol] {
        &self.pronunciation
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

impl FromStr for Rule {
    type Err = Error;

    /// Takes a line from the cmudict and turns it into a `Rule`.
    ///
    /// Format needs to be
    ///
    /// ```ignore
    /// WORD A B C
    /// ```
    ///
    fn from_str(s: &str) -> Result<Rule> {
        let mut iter = s.split_whitespace().filter(|s| !s.is_empty());
        let label = iter.next().ok_or(parse_error(&format!("Expected label, found EOF")))?;

        let symbols: Vec<_> = iter.map(|s| Symbol::from_str(s)).collect::<Result<Vec<_>>>()?;

        Ok(Rule::new(label.to_string(), symbols))
    }
}

#[cfg(test)]
mod tests {
    use super::{Symbol, Stress};
    use std::str::FromStr;

    #[test]
    fn test_consonant() {
        let dh = "DH";
        let converted = Symbol::from_str(dh);
        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), Symbol::DH);
    }

    #[test]
    fn test_vowel() {
        let aa = "AA1";
        let converted = Symbol::from_str(aa);
        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), Symbol::AA(Stress::Primary));
    }

    #[test]
    fn test_vec() {
        let v = vec!["AA1", "K", "L", "TH"];
        let converted = v.iter().map(|s| Symbol::from_str(s).unwrap()).collect::<Vec<_>>();
        assert_eq!(
                converted,
                vec![
                    Symbol::AA(Stress::Primary),
                    Symbol::K,
                    Symbol::L,
                    Symbol::TH,
                ]
        );
    }
}
