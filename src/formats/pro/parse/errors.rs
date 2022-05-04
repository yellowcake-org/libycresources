#[derive(Debug)]
pub enum Format {
    Type,
    Data,
    Flags,
    Consistency,
}

#[derive(Debug)]
pub enum Error {
    Read(std::io::Error),
    Format(Format),
    Source,
}