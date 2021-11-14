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
            pub message_id: u32,
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
            pub tile: ScaledValue<u16, u16>, // TODO: Coordinates!
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

            pub actions: std::collections::HashSet<super::common::actions::Instance>,
            pub material: super::common::Material,

            pub cost: u16,
            pub size: u16,
            pub weight: u16,

            pub is_hidden: bool,

            pub script_id: Option<u16>,
            pub sprite_id: u8,

            pub _sounds_ids: u16, // TODO: It represents multiple sounds, no info
        }

        pub mod armor {
            pub struct Threshold {}
            pub struct Resistance {}

            pub struct Instance {
                pub class: u32,
                pub threshold: Threshold,
                pub resistance: Resistance,

                pub perk_id: Option<u32>,
                pub male_sprite_id: Option<u32>,
                pub female_sprite_id: Option<u32>,
            }
        }

        pub mod container {
            pub enum Flags {
                NoPickUp,
                MagicHands,
            }

            pub struct Instance {
                pub size: u32,
                pub flags: std::collections::HashSet<Flags>,
            }
        }

        pub mod drug {
            use crate::common::types::ScaledValue;

            pub enum Amount {
                Fixed(u32),
                Random(std::ops::RangeInclusive<u32>),
            }

            pub struct Impact {
                pub amount: Amount,
                pub delay: Option<std::time::Duration>,
            }

            pub struct Effect {
                pub stat_id: u32,
                pub impacts: [Impact; 3],
            }

            pub struct Addiction {
                pub perk_id: Option<u32>,

                pub delay: std::time::Duration,
                pub chance: ScaledValue<u8, u8>,
            }

            pub struct Instance {
                pub effects: [Effect; 3],
                pub addiction: Addiction,
            }
        }

        pub mod weapon {
            pub mod damage {
                pub enum Type {
                    Default,
                    Laser,
                    Fire,
                    Plasma,
                    Electrical,
                    Emp,
                    Explosive,
                }

                pub struct Instance {
                    pub r#type: Type,
                    pub value: std::ops::RangeInclusive<u32>,
                }
            }

            pub struct Attack {
                pub cost: u16,
                pub range: std::ops::RangeInclusive<u16>,
            }

            pub struct Instance {
                pub strength: u16,
                pub failure: u16, // table?

                pub damage: damage::Instance,
                pub attacks: [Attack; 2],

                pub caliber_id: Option<u16>, //proto.msg - 300 + id
                pub burst_count: u16,
                pub ammo_count: u16,

                pub animation_code: u8,
                pub projectile_id: u32,
                pub perk_id: u32,
                pub ammo_id: u32,

                pub _sounds_ids: u16, // TODO: It represents multiple sounds, no info
            }
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
