pub mod extract;
pub mod list;

mod fetch;

#[derive(Debug)]
pub enum Error {
    Read(std::io::Error),
    Decode(std::string::FromUtf8Error),
    Decompress,
}

pub struct Entry {
    pub path: String,
    pub size: usize,
    pub range: std::ops::Range<usize>,
}
