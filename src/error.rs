use std::fmt::{Debug, Display, Formatter};
use std::num::ParseFloatError;
use std::string::FromUtf8Error;
use std::{io, result};
use log::SetLoggerError;

pub enum  Error {
    Message(Box<str>),
    Io(io::Error),
    SerdeJson(serde_json::Error),
    SevenzRust(sevenz_rust::Error),
    Curl(curl::Error),
    FromUtf8(FromUtf8Error),
    ParseFloat(ParseFloatError),
    SetLogger(SetLoggerError)
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Message(msg) => {
                write!(f, "{}", msg)
            }
            Error::Io(e) => {
                write!(f, "{:?}", e)
            }
            Error::SerdeJson(e) => {
                write!(f, "{:?}", e)
            }
            Error::SevenzRust(e) => {
                write!(f, "{:?}", e)
            }
            Error::Curl(e) => {
                write!(f, "{:?}", e)
            }
            Error::FromUtf8(e) => {
                write!(f, "{:?}", e)
            }
            Error::ParseFloat(e) => {
                write!(f, "{:?}", e)
            }
            Error::SetLogger(e) => {
                write!(f, "{:?}", e)
            }
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

pub type Result<T> = result::Result<T, Error>;

impl From<SetLoggerError> for Error{
    fn from(value: SetLoggerError) -> Self {
        Error::SetLogger(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeJson(value)
    }
}

impl From<curl::Error> for Error {
    fn from(value: curl::Error) -> Self {
        Error::Curl(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::FromUtf8(value)
    }
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Error::ParseFloat(value)
    }
}

impl From<sevenz_rust::Error> for Error {
    fn from(value: sevenz_rust::Error) -> Self {
        Error::SevenzRust(value)
    }
}

