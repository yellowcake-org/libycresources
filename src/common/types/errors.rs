use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Format,
    IO(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            Error::IO(io) => { write!(f, "{:?}", io) }
            Error::Format => { write!(f, "Invalid file format.") }
        };
    }
}