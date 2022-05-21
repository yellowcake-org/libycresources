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