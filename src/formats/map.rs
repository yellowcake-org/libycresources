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
    pub objects: HashSet<state::object::Instance>,
    pub blueprints: HashSet<state::blueprint::Instance>,
}

pub mod common {
    use std::collections::HashSet;

    use crate::common::types::ScaledValue;

    #[derive(Debug, Hash, Eq, PartialEq)]
    pub struct Position {
        pub x: ScaledValue<u8, std::ops::Range<u8>>,
        pub y: ScaledValue<u8, std::ops::Range<u8>>,
    }

    pub type Elevation = ScaledValue<u8, std::ops::Range<u8>>;
    pub type Orientation = ScaledValue<u8, std::ops::Range<u8>>;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Flag { Save }

    #[derive(Debug)]
    pub struct Variables {
        pub local: HashSet<i32>,
        pub global: HashSet<i32>,
    }
}

pub mod defaults {
    use crate::formats::map::common::{Elevation, Orientation, Position};

    #[derive(Debug)]
    pub struct Instance {
        pub position: Position,
        pub elevation: Elevation,
        pub orientation: Orientation,
    }
}

pub mod tiles {
    const SIDE_SIZE: usize = 100;

    type Surface = [[Option<u16>; SIDE_SIZE]; SIDE_SIZE];

    #[derive(Debug)]
    pub struct Instance {
        pub roof: Surface,
        pub floor: Surface,
    }
}

pub mod state {
    pub mod blueprint {
        #[derive(Debug, Hash, Eq, PartialEq)]
        pub enum Type {
            System,
            Spatial(spatial::Instance),
            Time(time::Instance),
            Item,
            Critter,
        }

        #[derive(Debug, Hash, Eq, PartialEq)]
        pub struct Connections {
            pub program_id: u32,
            pub object_id: Option<u32>,
        }

        #[derive(Debug, Hash, Eq, PartialEq)]
        pub struct Variables {
            pub offset: u32,
            pub count: u32,
        }

        #[derive(Debug, Hash, Eq, PartialEq)]
        pub struct Instance {
            pub id: u16,
            pub r#type: Type,
            pub variables: Option<Variables>,
            pub connections: Connections,
        }

        pub mod spatial {
            use crate::formats::map::common::{Elevation, Position};

            #[derive(Debug, Hash, Eq, PartialEq)]
            pub struct Instance {
                pub position: Position,
                pub distance: u16,
                pub elevation: Elevation,
            }
        }

        pub mod time {
            #[derive(Debug, Hash, Eq, PartialEq)]
            pub struct Instance {
                pub duration: std::time::Duration,
            }
        }
    }

    pub mod object {
        #[derive(Debug, Hash, Eq, PartialEq)]
        pub struct Instance {
            pub reference_id: u32,
        }
    }
}