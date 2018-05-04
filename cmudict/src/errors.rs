use failure::Error;

/// Shortcut for Result<T, failure::Error>
pub type Result<T> = ::std::result::Result<T, Error>;

/// Enum of possible errors
#[derive(Debug, Fail)]
pub enum Errors {
    /// Wraps a std::io::Error
    #[fail(display = "io error: {:?}", _0)]
    IoErr(::std::io::Error),
    /// Wraps a reqwest::Error
    #[fail(display = "reqwest error: {:?}", _0)]
    ReqwestErr(::reqwest::Error),
}
