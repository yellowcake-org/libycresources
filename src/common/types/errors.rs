#[derive(Debug)]
pub enum Error {
    Format,
    Read(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Read(err)
    }
}