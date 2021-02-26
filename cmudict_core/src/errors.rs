use thiserror::Error;

/// Enum for all possible parse errors
#[derive(Debug, Clone, Error, PartialEq)]
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

/// Shortcut for Result<T, failure::Error>
pub type Result<T> = ::std::result::Result<T, ParseError>;
