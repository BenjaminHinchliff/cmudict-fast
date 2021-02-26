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
    RuleParseError(#[from] cmudict_core::ParseError),
}

/// Shortcut for Result<T, failure::Error>
pub type Result<T> = ::std::result::Result<T, Error>;
