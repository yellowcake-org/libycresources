use std::collections::HashSet;

pub mod parse;
mod traits;

#[derive(Debug)]
pub struct Map {
    pub id: u32,
    pub version: u32,
    pub filename: String,

    pub flags: HashSet<common::Flag>,

    pub entrance: location::Grid,
    pub variables: common::Variables,

    pub ticks: u32,
    pub darkness: u32,

    pub tiles: [Option<tiles::Instance>; 3],
    pub scripts: Vec<blueprint::script::Instance>,
    pub prototypes: Vec<blueprint::prototype::Instance>,
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

pub mod location {
    use std::ops::{Range, RangeInclusive};

    use crate::common::types::geometry::{Coordinate, Elevation, Orientation};

    #[derive(Debug, Eq, PartialEq)]
    pub struct Grid {
        pub position: Coordinate<u8, Range<u8>>,
        pub elevation: Elevation,
        pub orientation: Orientation,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct Screen {
        pub position: Coordinate<i32, RangeInclusive<i32>>,
        pub correction: Coordinate<i32, RangeInclusive<i32>>,
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

pub mod blueprint {
    pub mod script {
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
            pub program_id: Option<u32>,
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
            use crate::common::types::geometry::{Coordinate, Elevation};

            #[derive(Debug, Hash, Eq, PartialEq)]
            pub struct Instance {
                pub position: Coordinate<u8, std::ops::Range<u8>>,
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

    pub mod prototype {
        use crate::common::types::models::Identifier;
        use crate::formats::frm::FrameIndex;
        use crate::formats::map::location;
        use crate::formats::pro::{meta, ObjectPatch, ObjectType};

        #[derive(Debug, Eq, PartialEq)]
        pub struct Instance {
            pub id: Identifier<ObjectType>,
            pub patch: Patch,
            pub location: Location,
            pub appearance: Appearance,
            pub inventory: inventory::Instance,
        }

        #[derive(Debug, Eq, PartialEq)]
        pub struct Patch {
            pub meta: meta::Patch,
            pub object: ObjectPatch,
        }

        #[derive(Debug, Eq, PartialEq)]
        pub struct Location {
            pub grid: Option<location::Grid>,
            pub screen: location::Screen,
        }

        #[derive(Debug, Eq, PartialEq)]
        pub struct Appearance {
            pub frame: Option<FrameIndex>,
            pub sprite_id: u32,
        }

        pub mod inventory {
            pub type Item = super::Instance;
            pub type Instance = Vec<Option<Item>>;
        }
    }
}