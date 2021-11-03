pub mod calculate;

use crate::common::graphics::*;

pub struct Palette {
    pub colors: [Option<ColorPixel>; 256],
}

pub struct AnimatedColors {
    pub values: Vec<ColorPixel>,
    pub frametime: std::time::Duration,
}

pub struct AnimatedPalette {
    pub alarm: AnimatedColors,
    pub slime: AnimatedColors,
    pub shore: AnimatedColors,
    pub screen: AnimatedColors,

    pub fire_slow: AnimatedColors,
    pub fire_fast: AnimatedColors,
}
