use std::string::String;

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
