pub mod extract;
pub mod list;

pub struct Entry {
    pub path: String,
    pub size: usize,
    pub range: std::ops::Range<usize>,
}
