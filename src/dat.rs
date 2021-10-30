pub mod extract;
pub mod parse;

pub struct File {
    pub name: String,
    pub size: usize,
    pub(crate) range: std::ops::Range<usize>,
}

pub struct Directory {
    pub name: String,
    pub files: Vec<File>,
    pub children: Vec<Directory>,
}
