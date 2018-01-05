use std::str::FromStr;
use std::error;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Stress {
    None,
    Primary,
    Secondary,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    AA(String, Stress),
    AH(String, Stress),
    AO(String, Stress),
    AW(String, Stress),
    AY(String, Stress),
    B(String),
    CH(String),
    D(String),
    DH(String),
    EH(String, Stress),
    ER(String, Stress),
    EY(String, Stress),
    F(String),
    G(String),
    HH(String),
    IH(String, Stress),
    IY(String, Stress),
    JH(String),
    K(String),
    L(String),
    M(String),
    N(String),
    NG(String),
    OW(String, Stress),
    OY(String, Stress),
    P(String),
    R(String),
    S(String),
    SH(String),
    T(String),
    TH(String),
    UH(String, Stress),
    UW(String, Stress),
    V(String),
    W(String),
    Y(String),
    Z(String),
    ZH(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    ParseError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "ParseError"
    }
}

macro_rules! parse_stress {
    ( $next:expr, $symbol:expr, $name:expr ) => {{
        match $next {
            Some('0') | None => Ok($symbol($name, Stress::None)),
            Some('1') => Ok($symbol($name, Stress::Primary)),
            Some('2') => Ok($symbol($name, Stress::Secondary)),
            Some(_) => Err(Error::ParseError),
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
            Symbol::AA(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::AH(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::AO(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::AW(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::AY(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::B(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::CH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::D(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::DH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::EH(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::ER(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::EY(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::F(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::G(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::HH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::IH(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::IY(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::JH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::K(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::L(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::M(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::N(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::NG(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::OW(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::OY(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::P(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::R(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::S(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::SH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::T(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::TH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::UH(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::UW(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::V(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::W(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::Y(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::Z(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::ZH(ref s1) => {
                write!(f, "{}", s1)
            },
        }
    }
}

impl FromStr for Symbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Symbol, Error> {
        let mut chrs = s.chars();

        match chrs.next() {
            None => Err(Error::ParseError),
            Some('A') => {
                match chrs.next() {
                    Some('A') => parse_stress!( chrs.next(), Symbol::AA, String::from(s) ),
                    Some('H') => parse_stress!( chrs.next(), Symbol::AH, String::from(s) ),
                    Some('O') => parse_stress!( chrs.next(), Symbol::AO, String::from(s) ),
                    Some('W') => parse_stress!( chrs.next(), Symbol::AW, String::from(s) ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::AY, String::from(s) ),
                    Some(_) | None => Err(Error::ParseError),
                }
            },
            Some('E') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::EH, String::from(s) ),
                    Some('R') => parse_stress!( chrs.next(), Symbol::ER, String::from(s) ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::EY, String::from(s) ),
                    Some(_) | None => Err(Error::ParseError),
                }
            },
            Some('I') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::IH, String::from(s) ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::IY, String::from(s) ),
                    Some(_) | None => Err(Error::ParseError),
                }
            },
            Some('O') => {
                match chrs.next() {
                    Some('W') => parse_stress!( chrs.next(), Symbol::OW, String::from(s) ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::OY, String::from(s) ),
                    Some(_) | None => Err(Error::ParseError),
                }
            },
            Some('U') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::UH, String::from(s) ),
                    Some('W') => parse_stress!( chrs.next(), Symbol::UW, String::from(s) ),
                    Some(_) | None => Err(Error::ParseError),
                }
            },
            Some('B') => Ok(Symbol::B(String::from(s))),
            Some('C') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::CH(String::from(s))),
                    Some(_) | None => Err(Error::ParseError),
                }
            },
            Some('D') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::DH(String::from(s))),
                    None => Ok(Symbol::D(String::from(s))),
                    Some(_) => Err(Error::ParseError),
                }
            },
            Some('F') => Ok(Symbol::F(String::from(s))),
            Some('G') => Ok(Symbol::G(String::from(s))),
            Some('H') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::HH(String::from(s))),
                    Some(_) | None => Err(Error::ParseError),
                }
            },
            Some('J') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::JH(String::from(s))),
                    Some(_) | None => Err(Error::ParseError),
                }
            },
            Some('K') => Ok(Symbol::K(String::from(s))),
            Some('L') => Ok(Symbol::L(String::from(s))),
            Some('M') => Ok(Symbol::M(String::from(s))),
            Some('N') => {
                match chrs.next() {
                    Some('G') => Ok(Symbol::NG(String::from(s))),
                    None => Ok(Symbol::N(String::from(s))),
                    Some(_) => Err(Error::ParseError),
                }
            },
            Some('P') => Ok(Symbol::P(String::from(s))),
            Some('R') => Ok(Symbol::R(String::from(s))),
            Some('S') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::SH(String::from(s))),
                    None => Ok(Symbol::S(String::from(s))),
                    Some(_) => Err(Error::ParseError),
                }
            },
            Some('T') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::TH(String::from(s))),
                    None => Ok(Symbol::T(String::from(s))),
                    Some(_) => Err(Error::ParseError),
                }
            },
            Some('V') => Ok(Symbol::V(String::from(s))),
            Some('W') => Ok(Symbol::W(String::from(s))),
            Some('Y') => Ok(Symbol::Y(String::from(s))),
            Some('Z') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::ZH(String::from(s))),
                    None => Ok(Symbol::Z(String::from(s))),
                    Some(_) => Err(Error::ParseError),
                }
            },
            Some(_) => Err(Error::ParseError),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::{Symbol, Stress};
    use std::str::FromStr;

    #[test]
    fn test_consonant() {
        let dh = "DH";
        let converted = Symbol::from_str(dh);
        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), Symbol::DH("DH".to_string()));
    }

    #[test]
    fn test_vowel() {
        let aa = "AA1";
        let converted = Symbol::from_str(aa);
        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), Symbol::AA("AA1".to_string(), Stress::Primary));
    }

    #[test]
    fn test_vec() {
        let v = vec!["AA1", "K", "L", "TH"];
        let converted = v.iter().map(|s| Symbol::from_str(s).unwrap()).collect::<Vec<_>>();
        assert_eq!(
                converted,
                vec![
                    Symbol::AA("AA1".to_string(), Stress::Primary),
                    Symbol::K("K".to_string()),
                    Symbol::L("L".to_string()),
                    Symbol::TH("TH".to_string()),
                ]
        );
    }
}
