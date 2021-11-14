pub mod builtin;

use crate::common::types::ScaledValue;

pub struct Pixel {
    pub red: ScaledValue<usize, usize>,
    pub green: ScaledValue<usize, usize>,
    pub blue: ScaledValue<usize, usize>,
}

pub struct AnimatedPixel {
    pub values: Vec<Pixel>,
    pub duration: std::time::Duration,
}
