use std::collections::HashSet;

pub mod parse;

pub struct Map {
    pub id: u32,
    pub version: u32,
    pub filename: String,

    pub defaults: common::Defaults,

    pub flags: HashSet<common::Flag>,
    pub elevations: HashSet<u8>,

    pub ticks: u32,
    pub darkness: u32,
}

pub mod common {
    use crate::common::types::ScaledValue;

    type Position = ScaledValue<u16, std::ops::Range<u16>>;
    type Elevation = ScaledValue<u8, std::ops::Range<u8>>;
    type Orientation = ScaledValue<u8, std::ops::Range<u8>>;

    pub struct Defaults {
        pub position: Position,
        pub elevation: Elevation,
        pub orientation: Orientation,
    }

    #[derive(PartialEq, Eq, Hash)]
    pub enum Flag { Save }
}