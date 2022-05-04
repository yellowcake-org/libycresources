use std::ops::Range;

use crate::common::types::ScaledValue;

pub mod builtin;

pub struct Pixel {
    pub red: ScaledValue<usize, Range<usize>>,
    pub green: ScaledValue<usize, Range<usize>>,
    pub blue: ScaledValue<usize, Range<usize>>,
}

pub struct AnimatedPixel {
    pub values: Vec<Pixel>,
    pub duration: std::time::Duration,
}
