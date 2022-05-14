use std::ops::Range;

use crate::common::types::geometry::Scaled;

pub mod builtin;

pub struct Pixel {
    pub red: Scaled<usize, Range<usize>>,
    pub green: Scaled<usize, Range<usize>>,
    pub blue: Scaled<usize, Range<usize>>,
}

pub struct AnimatedPixel {
    pub values: Vec<Pixel>,
    pub duration: std::time::Duration,
}
