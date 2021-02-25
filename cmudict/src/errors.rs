/// Enum of possible errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Wraps a std::io::Error
    #[error("io error: {0}")]
    IoErr(#[from] ::std::io::Error),
    #[error("invalid line {0}")]
    InvalidLine(usize),
    #[error("rule parse error: {0}")]
    RuleParseError(#[from] cmudict_core::ParseError),
}

/// Shortcut for Result<T, failure::Error>
pub type Result<T> = ::std::result::Result<T, Error>;
