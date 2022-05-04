pub mod parse;
mod traits;

pub struct Prototype {
    pub id: u16,
    pub meta: meta::Info,
    pub r#type: object::Type,
}

pub mod meta {
    use std::collections::HashSet;

    pub struct Info {
        pub light: info::Light,
        pub flags: HashSet<info::flags::Instance>,
        pub sprite: super::object::common::sprite::Reference,
        pub connections: info::Connections,
    }

    pub mod info {
        use std::ops::RangeInclusive;

        use crate::common::types::ScaledValue;

        pub struct Light {
            pub distance: ScaledValue<u8, RangeInclusive<u8>>,
            pub intensity: ScaledValue<u16, RangeInclusive<u16>>,
        }

        pub struct Connections {
            pub description_id: u32,
        }

        pub mod flags {
            #[derive(Debug, PartialEq, Eq, Hash)]
            pub enum Transparency {
                Wall,
                Glass,
                Steam,
                Energy,
                Red,
                End,
            }

            #[derive(Debug, PartialEq, Eq, Hash)]
            pub enum Instance {
                Flat,
                NotBlocking,
                NotBordered,
                MultiHex,
                ShotThrough,
                LightThrough,
                Transparency(Option<Transparency>),
            }
        }
    }
}

pub mod object {
    pub mod common {
        pub mod sprite {
            #[derive(Debug)]
            pub enum Type {
                Item,
                Critter,
                Scenery,
                Wall,
                Tile,
                Background,
                Interface,
                Inventory,
            }

            #[derive(Debug)]
            pub struct Reference {
                pub id: u16,
                pub r#type: Type,
            }
        }

        pub mod script {
            #[derive(Debug)]
            pub enum Type {
                Spatial,
                Item,
                Scenery,
                Critter,
            }

            #[derive(Debug)]
            pub struct Reference {
                pub id: u16,
                pub r#type: Type,
            }
        }

        pub mod world {
            #[derive(Debug)]
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

            #[derive(Debug, PartialEq, Eq, Hash)]
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
            #[derive(Debug)]
            pub enum Map {
                Local(u32),
                World,
            }

            #[derive(Debug)]
            pub enum Floor {
                Zero,
                First,
                Second,
            }

            #[derive(Debug)]
            pub struct Destination {
                // TODO: Coordinates!
                pub tile: u32,
                pub floor: Floor,
            }
        }

        pub mod critter {
            #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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
                DamageThresholdExplosive,
                DamageResistance,
                DamageResistanceLaser,
                DamageResistanceFire,
                DamageResistancePlasma,
                DamageResistanceElectrical,
                DamageResistanceEMP,
                DamageResistanceExplosive,
                RadiationResistance,
                PoisonResistance,
                Age,
                Gender,
                CurrentHitPoints,
                CurrentPoisonLevel,
                CurrentRadiationLevel,
            }

            #[derive(Debug)]
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

            #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

            #[derive(Debug, PartialEq, Eq, Hash)]
            pub enum Gender {
                Male,
                Female,
            }

            pub mod body {
                #[derive(Debug)]
                pub enum Type {
                    Biped,
                    Quadruped,
                    Robotic,
                }
            }
        }

        pub mod combat {
            pub mod damage {
                #[derive(Debug, PartialEq, Eq, Hash)]
                pub enum Type {
                    Default,
                    Laser,
                    Fire,
                    Plasma,
                    Electrical,
                    Emp,
                    Explosive,
                    Radiation,
                    Poison,
                }
            }
        }

        pub mod weapons {
            #[derive(Debug)]
            pub enum Caliber {
                Rocket,
                FlamethrowerFuel,
                CEnergyCell,
                DEnergyCell,
                Remington223,
                FiveMillimeter,
                SnW40,
                TenMillimeter,
                Magnum44,
                FourteenMillimeter,
                TwelveGauge,
                NineMillimeter,
                Bb,
                Unknown(u32),
            }
        }

        pub mod actions {
            #[derive(Debug, PartialEq, Eq, Hash)]
            pub struct Usage {
                pub itself: bool,
                pub something: bool,
                pub knees_down: bool,
            }

            #[derive(Debug, PartialEq, Eq, Hash)]
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
        use std::collections::HashSet;

        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum Flag {
            Hidden
        }

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
            pub _sounds_ids: u8, // TODO: It represents multiple sounds, no info
        }

        pub struct Instance {
            pub r#type: Type,
            pub flags: HashSet<Flag>,

            pub sprite: Option<super::common::sprite::Reference>,
            pub script: Option<super::common::script::Reference>,

            pub actions: HashSet<super::common::actions::Instance>,
            pub material: super::common::world::Material,

            pub size: u32,
            pub price: u32,
            pub weight: u32,

            pub connections: Connections,
        }

        pub mod armor {
            use std::collections::HashMap;

            use super::super::common::{combat::damage, critter, sprite};

            #[derive(Debug)]
            pub struct Appearance {
                pub sprites: HashMap<critter::Gender, sprite::Reference>,
            }

            pub struct Instance {
                pub class: u32,

                pub threshold: HashMap<damage::Type, u32>,
                pub resistance: HashMap<damage::Type, u32>,

                pub perk: Option<super::super::common::critter::Perk>,
                pub appearance: Appearance,
            }
        }

        pub mod container {
            use std::collections::HashSet;

            #[derive(Debug, PartialEq, Eq, Hash)]
            pub enum Flag {
                NoPickUp,
                MagicHands,
            }

            pub struct Instance {
                pub size: u32,
                pub flags: HashSet<Flag>,
            }
        }

        pub mod drug {
            use std::collections::HashMap;
            use std::ops::RangeInclusive;

            use crate::common::types::ScaledValue;
            use crate::formats::pro::object::common::critter::Statistic;

            #[derive(Debug)]
            pub enum Amount {
                Fixed(i32),
                Random(std::ops::RangeInclusive<i32>),
            }

            #[derive(Debug)]
            pub struct Effect {
                pub delay: Option<std::time::Duration>,
                pub impact: Amount,
            }

            #[derive(Debug)]
            pub struct Addiction {
                pub perk: super::super::common::critter::Perk,
                pub delay: std::time::Duration,
                pub chance: ScaledValue<u8, RangeInclusive<u8>>,
            }

            #[derive(Debug)]
            pub struct Instance {
                pub effects: HashMap<Statistic, [Effect; 3]>,
                pub addiction: Option<Addiction>,
            }
        }

        pub mod weapon {
            use std::collections::HashSet;

            #[derive(Debug, PartialEq, Eq, Hash)]
            pub enum Flag {
                BigGun,
                SecondHand,
            }

            #[derive(Debug)]
            pub struct Damage {
                pub value: std::ops::RangeInclusive<u32>,
                pub r#type: super::super::common::combat::damage::Type,
            }

            pub mod attack {
                #[derive(Debug, PartialEq, Eq, Hash)]
                pub enum Mode {
                    Punch,
                    Kick,
                    Swing,
                    Thrust,
                    Throw,
                    FireSingle,
                    FireBurst,
                    Flame,
                }

                #[derive(Debug, PartialEq, Eq, Hash)]
                pub struct Instance {
                    pub cost: u32,
                    pub mode: Mode,
                    pub range: std::ops::RangeInclusive<u32>,
                }
            }

            #[derive(Debug)]
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

            #[derive(Debug)]
            pub struct Rounds {
                pub burst: u32,
                pub magazine: u32,
            }

            #[derive(Debug)]
            pub struct Ammunition {
                pub rounds: Rounds,
                pub caliber: super::super::common::weapons::Caliber,
            }

            #[derive(Debug)]
            pub struct Requirements {
                pub strength: u32,
            }

            pub struct Connections {
                pub ammo_item_id: Option<u16>,
                pub failure_list_id: Option<u16>,
                pub projectile_misc_id: Option<u16>,

                pub _sounds_ids: u8,
            }

            pub struct Instance {
                pub flags: HashSet<Flag>,
                pub damage: Damage,
                pub attacks: [Option<attack::Instance>; 2],
                pub animation: Option<Animation>,
                pub requirements: Requirements,

                pub ammunition: Option<Ammunition>,

                pub perk: Option<super::super::common::critter::Perk>,
                pub connections: Connections,
            }
        }

        pub mod ammo {
            pub mod adjustments {
                pub struct Armor {
                    pub class: i32,
                    pub resistance: i32,
                }

                pub struct Damage {
                    pub divider: u32,
                    pub multiplier: u32,
                }

                pub struct Instance {
                    pub armor: Armor,
                    pub damage: Damage,
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
                pub power_item_id: Option<u32>,
            }

            pub struct Instance {
                pub count: u32,
                pub caliber: Option<super::super::common::weapons::Caliber>,
                pub connections: Connections,
            }
        }

        pub mod key {
            pub struct Instance {
                pub code: u32,
            }
        }
    }

    pub mod critter {
        use std::collections::{HashMap, HashSet};

        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum Flag {
            BarterAvailable,
            NoSteal,
            NoDrop,
            NoLimbsLoose,
            NoCorpseDisappear,
            NoAutoHeal,
            Invulnerable,
            NoCorpse,
            SpecialDeath,
            RangedMelee,
            NoKnockDown,
        }

        pub mod murder {
            #[derive(Debug)]
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

            #[derive(Debug)]
            pub struct Result {
                pub r#type: Type,
                pub experience: u32,
            }
        }

        pub struct Statistics {
            pub basic: HashMap<super::common::critter::Statistic, i32>,
            pub bonuses: HashMap<super::common::critter::Statistic, i32>,
        }

        pub struct Connections {
            pub ai_packet_id: u32,
        }

        pub struct Instance {
            pub team: u32,

            pub murder: murder::Result,
            pub damage: Option<super::common::combat::damage::Type>, // Fallout 2 only

            pub body: super::common::critter::body::Type,
            pub head: Option<super::common::sprite::Reference>,
            pub script: Option<super::common::script::Reference>,

            pub flags: HashSet<Flag>,
            pub skills: HashMap<super::common::critter::Skill, u32>,

            pub statistics: Statistics,
            pub connections: Connections,
        }
    }

    pub mod scenery {
        use std::collections::HashSet;

        pub enum Type {
            Door(door::Instance),
            Stairs(stairs::Instance),
            Elevator(elevator::Instance),
            Ladder(ladder::Instance),
            Generic(generic::Instance),
        }

        pub struct Connections {
            pub _sounds_ids: u8,
        }

        pub struct Instance {
            pub r#type: Type,

            pub light: HashSet<super::common::world::Light>,
            pub script: Option<super::common::script::Reference>,
            pub material: super::common::world::Material,

            pub actions: std::collections::HashSet<super::common::actions::Instance>,
            pub connections: Connections,
        }

        pub mod door {
            use std::collections::HashSet;

            #[derive(Debug, PartialEq, Eq, Hash)]
            pub enum Flags {
                Passable
            }

            pub struct Instance {
                pub flags: HashSet<Flags>,
                pub _unknown: u32,
            }
        }

        pub mod stairs {
            #[derive(Debug)]
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
                pub floor: u32,
                pub r#type: u32, // 0...23, hardcoded, do something?
            }
        }

        pub mod ladder {
            #[derive(Debug)]
            pub enum Direction {
                Top,
                Bottom,
            }

            pub struct Instance {
                pub direction: Direction,
                pub destination: super::super::common::map::Destination,
            }
        }

        pub mod generic {
            pub struct Instance {
                pub _unknown: u32,
            }
        }
    }

    pub mod wall {
        use std::collections::HashSet;

        pub struct Instance {
            pub light: HashSet<super::common::world::Light>,
            pub script: Option<super::common::script::Reference>,
            pub material: super::common::world::Material,

            pub actions: std::collections::HashSet<super::common::actions::Instance>,
        }
    }

    pub mod tile {
        pub struct Instance {
            pub material: super::common::world::Material,
        }
    }

    pub mod misc {
        pub struct Instance {
            pub _unknown: u32,
        }
    }
}
