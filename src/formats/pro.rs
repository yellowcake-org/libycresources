pub struct Prototype {
    pub id: u16,
    pub meta: meta::Info,
    pub r#type: object::Type,
}

pub mod meta {
    pub struct Info {
        pub light: info::Light,
        pub flags: std::collections::HashSet<info::flags::Instance>,

        pub connections: info::Connections,
    }

    pub mod info {
        use crate::common::types::ScaledValue;

        pub struct Light {
            pub distance: ScaledValue<u8, u8>,
            pub intensity: ScaledValue<u8, u8>,
        }

        pub struct Connections {
            pub sprite_id: u32,
            pub message_id: u16,
        }

        pub mod flags {
            pub enum Opacity {
                None,
                Wall,
                Glass,
                Steam,
                Energy,
                Red,
            }

            pub enum Instance {
                Flat,
                Blocking,
                Bordered,
                MultiHex,
                ShotThrough,
                LightThrough,
                Opaque(Opacity),
            }
        }
    }
}

pub mod object {
    pub mod common {
        use crate::common::types::ScaledValue;

        pub enum Material {
            Glass,
            Metal,
            Plastic,
            Wood,
            Dirt,
            Stone,
            Cement,
            Leather,
        }

        pub enum Light {
            Vertical,
            Horizontal,
            North,
            South,
            East,
            West,
        }

        pub struct Destination {
            pub tile: ScaledValue<u16, u16>,
            pub floor: ScaledValue<u16, u16>,
        }

        pub mod actions {
            pub struct Usage {
                pub itself: bool,
                pub something: bool,
                pub knees_down: bool,
            }

            pub enum Instance {
                Look,
                Talk,
                PickUp,
                Usage(Usage),
            }
        }
    }

    pub enum Type {
        Item(item::Instance),
        Critter(critter::Instance),
        Scenery(scenery::Instance),
        Wall(wall::Instance),
        Tile(tile::Instance),
        Misc(misc::Instance),
    }

    pub mod item {
        pub enum Type {
            Armor(armor::Instance),
            Container(container::Instance),
            Drug(drug::Instance),
            Weapon(weapon::Instance),
            Ammo(ammo::Instance),
            Misc(misc::Instance),
            Key(key::Instance),
        }

        pub struct Instance {
            pub r#type: Type,
        }

        pub mod armor {
            pub struct Instance {}
        }

        pub mod container {
            pub struct Instance {}
        }

        pub mod drug {
            pub struct Instance {}
        }

        pub mod weapon {
            pub struct Instance {}
        }

        pub mod ammo {
            pub struct Instance {}
        }

        pub mod misc {
            pub struct Instance {}
        }

        pub mod key {
            pub struct Instance {}
        }
    }

    pub mod critter {
        pub struct Instance {}
    }

    pub mod scenery {
        pub enum Type {
            Generic,
            Door(door::Instance),
            Stairs(stairs::Instance),
            Elevator(elevator::Instance),
            Ladder(ladder::Instance),
        }

        pub struct Instance {
            pub r#type: Type,
        }

        pub mod door {
            pub struct Instance {}
        }

        pub mod stairs {
            pub struct Instance {}
        }

        pub mod elevator {
            pub struct Instance {}
        }

        pub mod ladder {
            pub struct Instance {}
        }
    }

    pub mod wall {
        pub struct Instance {}
    }

    pub mod tile {
        pub struct Instance {}
    }

    pub mod misc {
        pub struct Instance {}
    }
}
