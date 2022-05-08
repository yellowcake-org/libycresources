use std::collections::HashSet;

pub mod parse;

#[derive(Debug)]
pub struct Map {
    pub id: u32,
    pub version: u32,
    pub filename: String,

    pub flags: HashSet<common::Flag>,

    pub defaults: defaults::Instance,
    pub variables: common::Variables,

    pub ticks: u32,
    pub darkness: u32,

    pub tiles: [Option<tiles::Instance>; 3],
}

pub mod common {
    use std::collections::HashSet;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Flag { Save }

    #[derive(Debug)]
    pub struct Variables {
        pub local: HashSet<i32>,
        pub global: HashSet<i32>,
    }
}

pub mod defaults {
    use crate::common::types::ScaledValue;

    #[derive(Debug)]
    pub struct Position {
        pub x: ScaledValue<u8, std::ops::Range<u8>>,
        pub y: ScaledValue<u8, std::ops::Range<u8>>
    }

    type Elevation = ScaledValue<u8, std::ops::Range<u8>>;
    type Orientation = ScaledValue<u8, std::ops::Range<u8>>;

    #[derive(Debug)]
    pub struct Instance {
        pub position: Position,
        pub elevation: Elevation,
        pub orientation: Orientation,
    }
}

pub mod tiles {
    type Surface = [[Option<u16>; 100]; 100];

    #[derive(Debug)]
    pub struct Instance {
        pub roof: Surface,
        pub floor: Surface,
    }
}