use std::fmt::{Display, Formatter};
use std::io;
use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(io::Error),
    LengthNotKnown,
    InvalidString,
}

impl Error {
    pub(crate) const fn io(err: io::Error) -> Self{
        Error::Io(err)
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self where T: Display {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self where T: Display {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::Io(ref err) => Display::fmt(err, f),
            Error::LengthNotKnown => f.write_str("length not known"),
            Error::InvalidString => f.write_str("invalid string"),
        }
    }
}

impl std::error::Error for Error {}
