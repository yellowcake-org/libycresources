use std::string::String;

pub struct Dirs {
    pub names: Vec<String>,
    pub(crate) offset: u64,
}

pub enum Size {
    Plain(u32),
    Packed { compressed: u32, plain: u32 },
}

pub struct File {
    pub name: String,
    pub path: String,
    pub size: Size,

    pub(crate) offset: u32,
}
