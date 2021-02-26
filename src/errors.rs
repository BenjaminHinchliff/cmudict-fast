/// Enum of possible errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Wraps a std::io::Error
    #[error("io error: {0}")]
    IoErr(#[from] ::std::io::Error),
    /// Handles case where label and pronunciation aren't separated
    #[error("line {0} is missing a space and so cannot be properly split")]
    InvalidLine(usize),
    #[error("rule parse error: {0}")]
    /// Wraps errors coming from parsing rules with cmudict_core
    RuleParseError(#[from] ParseError),
}

/// Shortcut for Result<T, errors::Error>
pub type Result<T> = ::std::result::Result<T, Error>;

/// Enum for all possible parse errors
#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum ParseError {
    /// An invalid stress marker was parsed
    #[error("Expected stress marker '0', '1', or '2', got {0}")]
    ExpectedStress(char),
    /// early eof when something else expected
    #[error("Expected {0}, got EOF")]
    UnexpectedEOF(&'static str),
    /// early eof when something else expected
    #[error("Expected {0} after {1}, got EOF")]
    UnexpectedEOFAfter(&'static str, &'static str),
    /// Invalid charcter found at start of phoneme 
    #[error("Expected {0}, got {1}")]
    UnexpectedCharacter(&'static str, char),
    /// Invalid character after start of phoneme
    #[error("Expected {0} after {1}, got {2}")]
    UnexpectedCharacterAfter(&'static str, &'static str, char),
}

/// Shortcut for Result<T, errors::ParseError>
pub type ParseResult<T> = ::std::result::Result<T, ParseError>;

