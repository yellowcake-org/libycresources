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
            pub description_id: u32,
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
        pub mod world {
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
                NorthCorner,
                SouthCorner,
                EastCorner,
                WestCorner,
            }
        }

        pub mod map {
            use crate::common::types::ScaledValue;

            pub struct Destination {
                pub tile: ScaledValue<u16, u16>, // TODO: Coordinates!
                pub floor: ScaledValue<u16, u16>,
            }
        }

        pub mod character {
            pub enum Statistic {
                Strength,
                Perception,
                Endurance,
                Charisma,
                Intelligence,
                Agility,
                Luck,
                MaximumHitPoints,
                MaximumActionPoints,
                ArmorClass,
                UnarmedDamage,
                MeleeDamage,
                CarryWeight,
                Sequence,
                HealingRate,
                CriticalChance,
                BetterCriticals,
                DamageThreshold,
                DamageThresholdLaser,
                DamageThresholdFire,
                DamageThresholdPlasma,
                DamageThresholdElectrical,
                DamageThresholdEMP,
                DamageThresholdExplosion,
                DamageResistance,
                DamageResistanceLaser,
                DamageResistanceFire,
                DamageResistancePlasma,
                DamageResistanceElectrical,
                DamageResistanceEMP,
                DamageResistanceExplosion,
                RadiationResistance,
                PoisonResistance,
                Age,
                Gender,
                CurrentHitPoints,
                CurrentPoisonLevel,
                CurrentRadiationLevel,
            }

            pub enum Perk {
                Awareness,
                BonusHtHAttacks,
                BonusHtHDamage,
                BonusMove,
                BonusRangedDamage,
                BonusRateOfFire,
                EarlierSequence,
                FasterHealing,
                MoreCriticals,
                NightVision,
                Presence,
                RadResistance,
                Toughness,
                StrongBack,
                Sharpshooter,
                SilentRunning,
                Survivalist,
                MasterTrader,
                Educated,
                Healer,
                FortuneFinder,
                BetterCriticals,
                Empathy,
                Slayer,
                Sniper,
                SilentDeath,
                ActionBoy,
                MentalBlock,
                Lifegiver,
                Dodger,
                Snakeater,
                MrFixit,
                Medic,
                MasterThief,
                Speaker,
                HeaveHo,
                FriendlyFoe,
                Pickpocket,
                Ghost,
                CultOfPersonality,
                Scrounger,
                Explorer,
                FlowerChild,
                Pathfinder,
                AnimalFriend,
                Scout,
                MysteriousStranger,
                Ranger,
                QuickPockets,
                SmoothTalker,
                SwiftLearner,
                Tag,
                Mutate,
                NukaColaAddiction,
                BuffoutAddiction,
                MentatsAddiction,
                PsychoAddiction,
                RadawayAddiction,
                WeaponLongRange,
                WeaponAccurate,
                WeaponPenetrate,
                WeaponKnockback,
                PoweredArmor,
                CombatArmor,
            }

            pub enum Gender {
                Male,
                Female,
            }
        }

        pub mod combat {
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
            }
        }

        pub mod weapons {
            pub enum Caliber {
                Rocket,
                FlamethrowerFuel,
                CEnergyCell,
                DEnergyCell,
                Remington223,
                FiveMillimeter,
                SnW40,
                TenMillimiter,
                Magnum44,
                FourteenMillimeter,
                TwelveGauge,
                NineMillimeter,
                Bb,
            }
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

        pub struct Connections {
            pub sprite_id: u32,
            pub script_id: Option<u32>,

            pub _sounds_ids: u32, // TODO: It represents multiple sounds, no info
        }

        pub struct Instance {
            pub r#type: Type,
            pub is_hidden: bool,

            pub actions: std::collections::HashSet<super::common::actions::Instance>,
            pub material: super::common::world::Material,

            pub cost: u16,
            pub size: u16,
            pub weight: u16,

            pub connections: Connections,
        }

        pub mod armor {
            pub struct Appearance {
                pub gender: super::super::common::character::Gender,
                pub sprite_id: u32,
            }

            pub struct Instance {
                pub class: u32,

                pub threshold: super::super::common::combat::damage::Type,
                pub resistance: super::super::common::combat::damage::Type,

                pub perk: Option<super::super::common::character::Perk>,
                pub appearance: Appearance,
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
                pub delay: Option<std::time::Duration>,
                pub amount: Amount,
            }

            pub struct Effect {
                pub impacts: [Impact; 3],
                pub statistic: super::super::common::character::Statistic,
            }

            pub struct Addiction {
                pub perk: Option<super::super::common::character::Perk>,

                pub delay: std::time::Duration,
                pub chance: ScaledValue<u8, u8>,
            }

            pub struct Instance {
                pub effects: [Effect; 3],
                pub addiction: Addiction,
            }
        }

        pub mod weapon {
            pub struct Damage {
                pub value: std::ops::RangeInclusive<u32>,
                pub r#type: super::super::common::combat::damage::Type,
            }

            pub struct Attack {
                pub cost: u16,
                pub range: std::ops::RangeInclusive<u16>,
            }

            pub enum Animation {
                Knife,
                Club,
                Sledgehammer,
                Spear,
                Pistol,
                SubmachineGun,
                Rifle,
                BigGun,
                Minigun,
                RocketLauncher,
            }

            pub struct Connections {
                pub ammo_item_idx: u32,
                pub projectile_misc_idx: u32,

                pub _sounds_ids: u16,
            }

            pub struct Instance {
                pub animation: Option<Animation>,

                pub min_strength: u16,
                pub failure_chance: u16, // table?

                pub damage: Damage,
                pub attacks: [Attack; 2],

                pub caliber: Option<super::super::common::weapons::Caliber>,

                pub capacity: u16,
                pub burst_count: u16,

                pub perk: Option<super::super::common::character::Perk>,
                pub connections: Connections,
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
