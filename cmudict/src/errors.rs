use failure::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Errors {
    #[fail(display = "io error: {:?}", _0)]
    IoErr(::std::io::Error),
    #[fail(display = "reqwest error: {:?}", _0)]
    ReqwestErr(::reqwest::Error),
}
