#[derive(Debug)]
pub enum Error {
    Read(std::io::Error),
    Format(Format),
    Source,
}

impl From<std::io::Error> for Error {
    fn from(io_err: std::io::Error) -> Self {
        Self::Read(io_err)
    }
}

#[derive(Debug)]
pub enum Format {
    Type,
    Data,
    Flags,
    Consistency,
}