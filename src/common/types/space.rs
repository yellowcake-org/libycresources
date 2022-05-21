use std::ops::Range;
use crate::common::types::geometry::Scaled;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Elevation {
    pub level: Scaled<u8, Range<u8>>,
}
