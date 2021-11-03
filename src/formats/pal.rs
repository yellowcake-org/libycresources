pub mod palette;
pub mod parse;

#[derive(Copy, Clone)]
pub struct RawPaletteColor {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

#[derive(Copy, Clone)]
pub struct RawPalette {
    pub(crate) colors: [RawPaletteColor; 256],
}
