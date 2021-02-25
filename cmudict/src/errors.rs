/// Enum of possible errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Wraps a std::io::Error
    #[error("io error: {0}")]
    IoErr(#[from] ::std::io::Error),
}

/// Shortcut for Result<T, failure::Error>
pub type Result<T> = ::std::result::Result<T, Error>;
