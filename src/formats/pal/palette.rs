pub mod calculate;

use crate::common::graphics::*;

pub struct Regular {
    pub colors: [Option<ColorPixel>; 256],
}

pub struct FrameColors {
    pub values: Vec<ColorPixel>,
    pub duration: std::time::Duration,
}

pub struct Animated {
    pub alarm: FrameColors,
    pub slime: FrameColors,
    pub shore: FrameColors,
    pub screen: FrameColors,

    pub fire_slow: FrameColors,
    pub fire_fast: FrameColors,
}
