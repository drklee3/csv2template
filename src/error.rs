use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

use tera::Error as TeraError;

pub type Result<T> = StdResult<T, Error>;

pub enum Error {
    Tera(TeraError),
    MissingArg(String),
}

impl From<TeraError> for Error {
    fn from(err: TeraError) -> Error {
        Error::Tera(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::Tera(ref inner) => inner.fmt(f),
            Error::MissingArg(ref inner) => write!(f, "Missing argument {}", inner),
        }
    }
}
