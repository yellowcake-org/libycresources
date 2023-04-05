use std::fmt::{Display, Formatter};

use ycresources::common::types::errors;

pub(crate) enum Error<'a> {
    Corrupted(&'a str),
    Internal(errors::Error, &'a str),
    IO(std::io::Error, &'a str),
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            Error::Corrupted(text) => { write!(f, "Contents are corrupted. {:?}", text) }
            Error::Internal(i, text) => { write!(f, "Internal error. {:?} {:?}", text, i) }
            Error::IO(io, text) => { write!(f, "IO error. {:?} {:?}", text, io) }
        };
    }
}