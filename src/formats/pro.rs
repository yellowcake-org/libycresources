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
            pub enum Map {
                Local(u32),
                World,
            }

            pub struct Destination {
                pub tile: u16, // TODO: Coordinates!
                pub floor: u8,
            }
        }

        pub mod critter {
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

            pub enum Skill {
                SmallGuns,
                BigGuns,
                EnergyWeapons,
                Unarmed,
                MeleeWeapons,
                Throwing,
                FirstAid,
                Doctor,
                Sneak,
                Lockpick,
                Steal,
                Traps,
                Science,
                Repair,
                Speech,
                Barter,
                Gambling,
                Outdoorsman,
            }

            pub enum Gender {
                Male,
                Female,
            }

            pub mod body {
                pub enum Type {
                    Biped,
                    Quadruped,
                    Robotic,
                }
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
        Misc,
    }

    pub mod item {
        pub enum Type {
            Armor(armor::Instance),
            Container(container::Instance),
            Drug(drug::Instance),
            Weapon(weapon::Instance),
            Ammo(ammo::Instance),
            Misc(misc::Instance),
            Key,
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
            use super::super::common::combat::damage;
            use std::collections::HashMap;

            pub struct Appearance {
                pub gender: super::super::common::critter::Gender,
                pub sprite_id: u32,
            }

            pub struct Instance {
                pub class: u8,

                pub threshold: HashMap<damage::Type, u16>,
                pub resistance: HashMap<damage::Type, u16>,

                pub perk: Option<super::super::common::critter::Perk>,
                pub appearance: Appearance,
            }
        }

        pub mod container {
            pub enum Flag {
                NoPickUp,
                MagicHands,
            }

            pub struct Instance {
                pub size: u32,
                pub flags: std::collections::HashSet<Flag>,
            }
        }

        pub mod drug {
            use crate::common::types::ScaledValue;

            pub enum Amount {
                Fixed(u16),
                Random(std::ops::RangeInclusive<u16>),
            }

            pub struct Impact {
                pub delay: Option<std::time::Duration>,
                pub amount: Amount,
            }

            pub struct Effect {
                pub impacts: [Impact; 3],
                pub statistic: super::super::common::critter::Statistic,
            }

            pub struct Addiction {
                pub perk: Option<super::super::common::critter::Perk>,

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

                pub perk: Option<super::super::common::critter::Perk>,
                pub connections: Connections,
            }
        }

        pub mod ammo {
            pub mod adjustments {
                pub struct Armor {
                    pub class: u32,

                    pub resistance: u32,
                    pub resistance_divider: u32,
                    pub resistance_multiplier: u32,
                }

                pub struct Instance {
                    pub armor: Armor,
                }
            }

            pub struct Instance {
                pub count: u32,
                pub caliber: Option<super::super::common::weapons::Caliber>,
                pub adjustments: adjustments::Instance,
            }
        }

        pub mod misc {
            pub struct Connections {
                pub power_items_idx: u32,
            }

            pub struct Instance {
                pub count: u32,
                pub caliber: super::super::common::weapons::Caliber,
                pub connections: Connections,
            }
        }
    }

    pub mod critter {
        use std::collections::{HashMap, HashSet};

        pub enum Flag {
            Barter,
            Steal,
            Drop,
            Limbs,
            Ages,
            Heal,
            Invulnerable,
            Flatten,
            Special,
            Range,
            Knock,
        }

        pub mod murder {
            pub enum Type {
                Men,
                Women,
                Children,
                SuperMutants,
                Ghouls,
                Brahmin,
                Radscorpions,
                Rats,
                Floaters,
                Centaurs,
                Robots,
                Dogs,
                Manti,
                DeathClaws,
                Plants,
                Geckos,
                Aliens,
                GiantAnts,
                BigBadBoss,
            }

            pub struct Result {
                pub r#type: Type,
                pub experience: u16,
            }
        }

        pub struct Parameters {
            pub age: u8,
            pub gender: super::common::critter::Gender,

            pub statistics: HashMap<super::common::critter::Statistic, u16>,
            pub threshold: HashMap<super::common::combat::damage::Type, u16>,
            pub resistance: HashMap<super::common::combat::damage::Type, u16>,
        }

        pub struct Connections {
            pub sprite_id: Option<u32>,
            pub script_id: Option<u32>,
            pub ai_packet_idx: u32,
        }

        pub struct Instance {
            pub team: u32,

            pub murder: murder::Result,
            pub damage: super::common::combat::damage::Type,

            pub body: super::common::critter::body::Type,
            pub flags: HashSet<Flag>,
            pub skills: HashSet<super::common::critter::Skill>,
            pub actions: HashSet<super::common::actions::Instance>,

            pub basic: Parameters,
            pub bonuses: Parameters,

            pub connections: Connections,
        }
    }

    pub mod scenery {
        pub enum Type {
            Generic,
            Door(door::Instance),
            Stairs(stairs::Instance),
            Elevator(elevator::Instance),
            Ladder(ladder::Instance),
        }

        pub struct Connections {
            pub script_id: u32,
            pub _sounds_ids: u32,
        }

        pub struct Instance {
            pub r#type: Type,

            pub light: super::common::world::Light,
            pub material: super::common::world::Material,

            pub actions: std::collections::HashSet<super::common::actions::Instance>,
            pub connections: Connections,
        }

        pub mod door {
            pub struct Instance {
                pub can_pass: bool,
            }
        }

        pub mod stairs {
            pub struct Destination {
                pub map: super::super::common::map::Map,
                pub target: super::super::common::map::Destination,
            }

            pub struct Instance {
                pub destination: Destination,
            }
        }

        pub mod elevator {
            pub struct Instance {
                pub floor: u8,
                pub r#type: u8, // 0...23, hardcoded, do something?
            }
        }

        pub mod ladder {
            pub enum Direction {
                Top,
                Bottom,
            }

            pub struct Instance {
                pub direction: Direction,
                pub destination: super::super::common::map::Destination,
            }
        }
    }

    pub mod wall {
        pub struct Connections {
            pub script_id: u32,
        }

        pub struct Instance {
            pub light: super::common::world::Light,
            pub material: super::common::world::Material,

            pub actions: std::collections::HashSet<super::common::actions::Instance>,
            pub connections: Connections,
        }
    }

    pub mod tile {
        pub struct Instance {
            pub material: super::common::world::Material,
        }
    }
}
