use std::ops::Range;

use crate::common::types::geometry::Scaled;

pub mod parse;

pub struct Glyph {
    pub width: u16,
    pub height: u16,

    pub dots: Vec<Scaled<usize, Range<usize>>>,
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
