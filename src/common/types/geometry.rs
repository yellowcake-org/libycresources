use std::ops::Range;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Scaled<V, S> {
    pub value: V,
    pub scale: S,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Coordinate<V, S> {
    pub x: Scaled<V, S>,
    pub y: Scaled<V, S>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Orientation {
    pub value: Scaled<u8, Range<u8>>,
}