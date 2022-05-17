use std::collections::HashSet;

pub mod parse;
mod traits;

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
    pub scripts: HashSet<blueprint::script::Instance>,
    pub prototypes: HashSet<blueprint::prototype::Instance>,
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
    use crate::common::types::geometry::{Coordinate, Elevation, Orientation};

    #[derive(Debug)]
    pub struct Instance {
        pub position: Coordinate<u8, std::ops::Range<u8>>,
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
        use crate::formats::pro;

        #[derive(Debug, Hash, Eq, PartialEq)]
        pub struct Instance {
            pub identifier: Identifier<pro::ObjectType>,
            // pub patch: Option<pro::patch::Instance>,
        }
    }
}