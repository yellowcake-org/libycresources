pub mod parse;

use super::super::common::graphics::ColorPixel;

pub struct RotatedColors {
    pub range: std::ops::Range<usize>,
    pub frametime: std::time::Duration,
}

pub struct Palette {
    pub colors: [Option<ColorPixel>; 256],

    pub alarm: RotatedColors,
    pub slime: RotatedColors,
    pub shore: RotatedColors,
    pub screen: RotatedColors,

    pub fire_slow: RotatedColors,
    pub fire_fast: RotatedColors,
}
