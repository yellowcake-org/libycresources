use std::ops::RangeInclusive;

use crate::common::types::geometry::Scaled;

pub mod builtin;

pub struct Pixel<P> {
    pub red: Scaled<P, RangeInclusive<P>>,
    pub green: Scaled<P, RangeInclusive<P>>,
    pub blue: Scaled<P, RangeInclusive<P>>,
}

pub struct AnimatedPixel {
    pub values: Vec<Pixel<u8>>,
    pub duration: std::time::Duration,
}
