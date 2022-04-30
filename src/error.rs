use serde::{de, ser};
use std::error;
use std::fmt::{self, Display};
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(io::Error),
    LengthNotKnown,
    InvalidString,
    InvalidChar,
    TrailingBytes,
    EofWhileDeserializing,
}

impl Error {
    pub(crate) fn io(err: io::Error) -> Self {
        if err.kind() == io::ErrorKind::UnexpectedEof {
            Error::EofWhileDeserializing
        } else {
            Error::Io(err)
        }
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::Io(ref err) => Display::fmt(err, f),
            Error::LengthNotKnown => f.write_str("length not known"),
            Error::InvalidString => f.write_str("invalid string"),
            Error::InvalidChar => f.write_str("invalid char"),
            Error::TrailingBytes => f.write_str("trailing bytes"),
            Error::EofWhileDeserializing => f.write_str("EOF while deserializing"),
        }
    }
}

impl error::Error for Error {}
