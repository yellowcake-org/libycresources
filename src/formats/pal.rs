pub mod parse;

use super::super::common::graphics::ColorPixel;

pub struct AnimatedColors {
    pub values: Vec<ColorPixel>,
    pub frametime: std::time::Duration,
}

pub struct Palette {
    pub colors: [Option<ColorPixel>; 256],

    pub alarm: AnimatedColors,
    pub slime: AnimatedColors,
    pub shore: AnimatedColors,
    pub screen: AnimatedColors,

    pub fire_slow: AnimatedColors,
    pub fire_fast: AnimatedColors,
}
