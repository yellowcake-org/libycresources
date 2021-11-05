pub mod parse;

use super::super::common::graphics::Pixel;

pub struct Glyph {
    pub width: u16,
    pub height: u16,

    pub pixels: Vec<Pixel>,
}

pub struct Spacing {
    pub vertical: u16,
    pub horizontal: u16,
}

pub struct Font {
    pub height: u16,
    pub spacing: Spacing,

    pub glyphs: [Glyph; 256],
}
