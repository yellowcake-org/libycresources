pub mod parse;

use crate::common::types::ScaledValue;

pub struct Glyph {
    pub width: u16,
    pub height: u16,

    pub dots: Vec<ScaledValue<usize, usize>>,
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
