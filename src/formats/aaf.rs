pub mod parse;

use super::super::common::graphics::Pixel;

pub struct Glyph {
    pub width: usize,
    pub height: usize,

    pub pixels: Vec<Pixel>,
}

pub struct Spacing {
    pub vertical: usize,
    pub horizontal: usize,
}

pub struct Font {
    pub height: usize,
    pub spacing: Spacing,

    pub glyphs: [Glyph; 256],
}
