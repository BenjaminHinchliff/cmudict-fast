use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ParseError {
    #[error("Expected stress marker '0', '1', or '2', got {0}")]
    ExpectedStress(char),
    #[error("Expected {0}, got EOF")]
    UnexpectedEOF(&'static str),
    #[error("Expected {0} after {1}, got EOF")]
    UnexpectedEOFAfter(&'static str, &'static str),
    #[error("Expected {0}, got {1}")]
    UnexpectedCharacter(&'static str, char),
    #[error("Expected {0} after {1}, got {2}")]
    UnexpectedCharacterAfter(&'static str, &'static str, char),
}

/// Shortcut for Result<T, failure::Error>
pub type Result<T> = ::std::result::Result<T, ParseError>;
