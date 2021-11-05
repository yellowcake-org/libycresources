pub mod extract;
pub mod parse;

pub struct File {
    pub name: String,
    pub size: u32,
    pub(crate) range: std::ops::Range<u32>,
}

pub struct Directory {
    pub name: String,
    pub files: Vec<File>,
    pub children: Vec<Directory>,
}
