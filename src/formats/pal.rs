pub mod palette;
pub mod parse;

#[derive(Copy, Clone)]
pub(crate) struct RawColorValue {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

#[derive(Copy, Clone)]
pub struct RawColorValues {
    pub(crate) values: [RawColorValue; 256],
}
