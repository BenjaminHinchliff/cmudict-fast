//! Core part of the cmudict crate
//!
//! This crate contains the logic to parse & construct "rules" from the cmudict text database
#![deny(missing_docs)]

use std::str::FromStr;
use std::fmt;

#[cfg(feature = "serialization")]
use serde::{Serialize, Deserialize};

use super::errors::{ParseError, ParseResult};

/// Used by a symbol to indicate what kind of stress it has
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[allow(missing_docs)]
pub enum Stress {
    None,
    Primary,
    Secondary,
}

/// Represents a single sound
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[allow(missing_docs)]
pub enum Symbol {
    AA(Stress),
    AE(Stress),
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

impl Symbol {
    /// Returns `true` if the symbol has primary stress
    pub fn is_primary(&self) -> bool {
        use self::Symbol::*;
        match self {
            | AA(Stress::Primary)
            | AE(Stress::Primary)
            | AH(Stress::Primary)
            | AO(Stress::Primary)
            | AW(Stress::Primary)
            | AY(Stress::Primary)
            | EH(Stress::Primary)
            | ER(Stress::Primary)
            | EY(Stress::Primary)
            | IH(Stress::Primary)
            | IY(Stress::Primary)
            | OW(Stress::Primary)
            | OY(Stress::Primary)
            | UH(Stress::Primary)
            | UW(Stress::Primary) => true,
            _ => false
        }
    }

    /// Returns `true` if the symbol has secondary stress
    pub fn is_secondary(&self) -> bool {
        use self::Symbol::*;
        match self {
            | AA(Stress::Secondary)
            | AE(Stress::Secondary)
            | AH(Stress::Secondary)
            | AO(Stress::Secondary)
            | AW(Stress::Secondary)
            | AY(Stress::Secondary)
            | EH(Stress::Secondary)
            | ER(Stress::Secondary)
            | EY(Stress::Secondary)
            | IH(Stress::Secondary)
            | IY(Stress::Secondary)
            | OW(Stress::Secondary)
            | OY(Stress::Secondary)
            | UH(Stress::Secondary)
            | UW(Stress::Secondary) => true,
            _ => false
        }
    }

    /// Returns `true` if the symbol has no stress
    pub fn is_unstressed(&self) -> bool {
        use self::Symbol::*;
        match self {
            | AA(Stress::Primary)
            | AE(Stress::Primary)
            | AH(Stress::Primary)
            | AO(Stress::Primary)
            | AW(Stress::Primary)
            | AY(Stress::Primary)
            | EH(Stress::Primary)
            | ER(Stress::Primary)
            | EY(Stress::Primary)
            | IH(Stress::Primary)
            | IY(Stress::Primary)
            | OW(Stress::Primary)
            | OY(Stress::Primary)
            | UH(Stress::Primary)
            | UW(Stress::Primary) => false,
            | AA(Stress::Secondary)
            | AE(Stress::Secondary)
            | AH(Stress::Secondary)
            | AO(Stress::Secondary)
            | AW(Stress::Secondary)
            | AY(Stress::Secondary)
            | EH(Stress::Secondary)
            | ER(Stress::Secondary)
            | EY(Stress::Secondary)
            | IH(Stress::Secondary)
            | IY(Stress::Secondary)
            | OW(Stress::Secondary)
            | OY(Stress::Secondary)
            | UH(Stress::Secondary)
            | UW(Stress::Secondary) => false,
            _ => true
        }
    }

    /// Returns `true` if the symbol is stressed in some way
    pub fn is_syllable(&self) -> bool {
        use self::Symbol::*;
        match self {
            | AA(..)
            | AE(..)
            | AH(..)
            | AO(..)
            | AW(..)
            | AY(..)
            | EH(..)
            | ER(..)
            | EY(..)
            | IH(..)
            | IY(..)
            | OW(..)
            | OY(..)
            | UH(..)
            | UW(..) => true,
            _ => false
        }
    }
}

macro_rules! parse_stress {
    ( $next:expr, $symbol:expr ) => {{
        match $next {
            Some('0') | None => Ok($symbol(Stress::None)),
            Some('1') => Ok($symbol(Stress::Primary)),
            Some('2') => Ok($symbol(Stress::Secondary)),
            Some(c) => Err(ParseError::ExpectedStress(c)),
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
            Symbol::AE(ref s1) => {
                write!(f, "AE{}", s1)
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
    type Err = ParseError;

    fn from_str(s: &str) -> ParseResult<Symbol> {
        let mut chrs = s.chars();

        match chrs.next() {
            None => Err(ParseError::UnexpectedEOF("character")),
            Some('A') => {
                match chrs.next() {
                    Some('A') => parse_stress!( chrs.next(), Symbol::AA ),
                    Some('E') => parse_stress!( chrs.next(), Symbol::AE ),
                    Some('H') => parse_stress!( chrs.next(), Symbol::AH ),
                    Some('O') => parse_stress!( chrs.next(), Symbol::AO ),
                    Some('W') => parse_stress!( chrs.next(), Symbol::AW ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::AY ),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("A, E, H, O, W, or Y", "A", c)),
                    None => Err(ParseError::UnexpectedEOFAfter("A, E, H, O, W, or Y", "A")),
                }
            },
            Some('E') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::EH ),
                    Some('R') => parse_stress!( chrs.next(), Symbol::ER ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::EY ),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("H, R, or Y", "E", c)),
                    None => Err(ParseError::UnexpectedEOFAfter("H, R, or Y", "E")),
                }
            },
            Some('I') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::IH ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::IY ),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("H or Y", "I", c)),
                    None => Err(ParseError::UnexpectedEOFAfter("H or Y", "I")),
                }
            },
            Some('O') => {
                match chrs.next() {
                    Some('W') => parse_stress!( chrs.next(), Symbol::OW ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::OY ),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("W or Y", "O", c)),
                    None => Err(ParseError::UnexpectedEOFAfter("W or Y", "O")),
                }
            },
            Some('U') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::UH ),
                    Some('W') => parse_stress!( chrs.next(), Symbol::UW ),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("H or W", "U", c)),
                    None => Err(ParseError::UnexpectedEOFAfter("H or W", "U")),
                }
            },
            Some('B') => Ok(Symbol::B),
            Some('C') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::CH),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("H", "C", c)),
                    None => Err(ParseError::UnexpectedEOFAfter("H", "C")),
                }
            },
            Some('D') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::DH),
                    None => Ok(Symbol::D),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("H or EOF", "D", c)),
                }
            },
            Some('F') => Ok(Symbol::F),
            Some('G') => Ok(Symbol::G),
            Some('H') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::HH),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("H", "H", c)),
                    None => Err(ParseError::UnexpectedEOFAfter("H", "H")),
                }
            },
            Some('J') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::JH),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("H", "J", c)),
                    None => Err(ParseError::UnexpectedEOFAfter("H", "J")),
                }
            },
            Some('K') => Ok(Symbol::K),
            Some('L') => Ok(Symbol::L),
            Some('M') => Ok(Symbol::M),
            Some('N') => {
                match chrs.next() {
                    Some('G') => Ok(Symbol::NG),
                    None => Ok(Symbol::N),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("G or EOF", "N", c)),
                }
            },
            Some('P') => Ok(Symbol::P),
            Some('R') => Ok(Symbol::R),
            Some('S') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::SH),
                    None => Ok(Symbol::S),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("H or EOF", "S", c)),
                }
            },
            Some('T') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::TH),
                    None => Ok(Symbol::T),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("H or EOF", "T", c)),
                }
            },
            Some('V') => Ok(Symbol::V),
            Some('W') => Ok(Symbol::W),
            Some('Y') => Ok(Symbol::Y),
            Some('Z') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::ZH),
                    None => Ok(Symbol::Z),
                    Some(c) => Err(ParseError::UnexpectedCharacterAfter("H or EOF", "Z", c)),
                }
            },
            Some(c) => Err(ParseError::UnexpectedCharacter("A-Z", c)),
        }
    }
}

/// Represents the complete pronunciation of a single word in the database
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Rule {
    label: String,
    pronunciation: Vec<Symbol>,
}


impl Rule {
    #[doc(hidden)]
    pub fn new<I: Into<String>>(label: I, pronunciation: Vec<Symbol>) -> Rule {
        Rule {
            label: label.into(),
            pronunciation: pronunciation,
        }
    }

    /// Returns `true` if the Rule has only one stressed `Symbol`
    pub fn is_monosyllabic(&self) -> bool {
        self.pronunciation.iter().filter(|s| s.is_syllable()).count() < 2
    }

    /// Retuns a slice of the Symbols for the word
    pub fn pronunciation(&self) -> &[Symbol] {
        &self.pronunciation
    }

    /// Returns the word
    pub fn label(&self) -> &str {
        &self.label
    }
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> ParseResult<Rule> {
        let mut iter = s.split_whitespace().filter(|s| !s.is_empty());
        let label = iter.next().ok_or(ParseError::UnexpectedEOF("label"))?;

        let symbols: Vec<_> = iter.map(|s| Symbol::from_str(s)).collect::<ParseResult<Vec<_>>>()?;

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
