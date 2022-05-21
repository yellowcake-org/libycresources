use std::collections::HashSet;

pub mod parse;

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

    pub tiles: Vec<tiles::Group>,
    pub scripts: Vec<blueprint::script::Instance>,
    pub prototypes: Vec<blueprint::prototype::Instance>,
}

pub mod common {
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Flag { Save }

    #[derive(Debug)]
    pub struct Variables {
        pub local: Vec<i32>,
        pub global: Vec<i32>,
    }
}

pub mod location {
    use std::ops::{Range, RangeInclusive};

    use crate::common::types::geometry::{Coordinate, Orientation};
    use crate::common::types::space::Elevation;

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
    use std::ops::Range;

    use crate::common::types::geometry::Coordinate;
    use crate::common::types::space::Elevation;

    #[derive(Debug, Eq, PartialEq)]
    pub struct Instance {
        pub id: u16,
        pub position: Coordinate<u8, Range<u8>>,
    }

    #[derive(Debug)]
    pub struct Group {
        pub floor: Vec<Instance>,
        pub ceiling: Vec<Instance>,
        pub elevation: Elevation,
    }
}

pub mod blueprint {
    pub mod script {
        // #[derive(Debug, Hash, Eq, PartialEq)]
        // pub enum Type {
        //     System,
        //     Spatial(spatial::Instance),
        //     Time(time::Instance),
        //     Item,
        //     Critter,
        // }

        use crate::common::types::models::script::Kind;

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
            pub kind: Kind<(), spatial::Instance, time::Instance, (), ()>,
            pub variables: Option<Variables>,
            pub connections: Connections,
        }

        pub mod spatial {
            use crate::common::types::geometry::Coordinate;
            use crate::common::types::space::Elevation;

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
        use crate::common::types::models;
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
            pub sprite: Identifier<models::sprite::Kind>,
        }

        pub mod inventory {
            pub type Item = super::Instance;
            pub type Instance = Vec<Option<Item>>;
        }
    }
}