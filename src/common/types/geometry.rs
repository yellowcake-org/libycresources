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
    pub value: Scaled<u8, std::ops::Range<u8>>,
}

// TODO: Consider moving it somewhere?
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Elevation {
    pub level: Scaled<u8, std::ops::Range<u8>>,
}