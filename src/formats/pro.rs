pub mod parse;

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
        pub sprite: super::object::common::sprite::Instance,
        pub connections: info::Connections,
    }

    pub mod info {
        use crate::common::types::ScaledValue;

        pub struct Light {
            pub distance: ScaledValue<u8, u8>,
            pub intensity: ScaledValue<u16, u16>,
        }

        pub struct Connections {
            pub description_id: u32,
        }

        pub mod flags {
            #[derive(PartialEq, Eq, Hash)]
            pub enum Transparency {
                Wall,
                Glass,
                Steam,
                Energy,
                Red,
            }

            #[derive(PartialEq, Eq, Hash)]
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

            pub struct Instance {
                pub id: u16,
                pub r#type: Type,
            }
        }

        pub mod script {
            pub enum Type {
                Spatial,
                Item,
                Scenery,
                Critter
            }

            pub struct Instance {
                pub id: u16,
                pub r#type: Type,
            }
        }

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
                // TODO: Coordinates!
                pub tile: u16,
                pub floor: u8,
            }
        }

        pub mod critter {
            #[derive(PartialEq, Eq, Hash, Copy, Clone)]
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

            #[derive(PartialEq, Eq, Hash)]
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
                #[derive(PartialEq, Eq, Hash)]
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
            #[derive(PartialEq, Eq, Hash)]
            pub struct Usage {
                pub itself: bool,
                pub something: bool,
                pub knees_down: bool,
            }

            #[derive(PartialEq, Eq, Hash)]
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
        use std::collections::HashSet;

        #[derive(PartialEq, Eq, Hash)]
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

            pub sprite: super::common::sprite::Instance,
            pub script: Option<super::common::script::Instance>,

            pub actions: HashSet<super::common::actions::Instance>,
            pub material: super::common::world::Material,

            pub size: u32,
            pub price: u32,
            pub weight: u32,

            pub connections: Connections,
        }

        pub mod armor {
            use super::super::common::critter::Gender;
            use super::super::common::combat::damage;
            use super::super::common::sprite;

            use std::collections::HashMap;

            pub struct Appearance {
                pub sprites: HashMap<Gender, sprite::Instance>,
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

            #[derive(PartialEq, Eq, Hash)]
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
            use crate::common::types::ScaledValue;

            pub enum Amount {
                Fixed(u32),
                Random(std::ops::RangeInclusive<u32>),
            }

            pub struct Effect {
                pub delay: Option<std::time::Duration>,
                pub impact: std::collections::HashMap<super::super::common::critter::Statistic, Amount>,
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
            use std::collections::HashSet;

            #[derive(PartialEq, Eq, Hash)]
            pub enum Flag {
                BigGun,
                SecondHand,
            }

            pub struct Damage {
                pub value: std::ops::RangeInclusive<u32>,
                pub r#type: super::super::common::combat::damage::Type,
            }

            pub mod attack {
                #[derive(PartialEq, Eq, Hash)]
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

                #[derive(PartialEq, Eq, Hash)]
                pub struct Instance {
                    pub cost: u32,
                    pub mode: Option<Mode>,
                    pub range: std::ops::RangeInclusive<u32>,
                }
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

            pub struct Rounds {
                pub burst: u32,
                pub magazine: u32,
            }

            pub struct Requirements {
                pub strength: u32,
            }

            pub struct Connections {
                pub ammo_item_id: u32,
                pub failure_list_id: u32,
                pub projectile_misc_id: u16,

                pub _sounds_ids: u8,
            }

            pub struct Instance {
                pub flags: HashSet<Flag>,
                pub damage: Damage,
                pub attacks: [attack::Instance; 2],
                pub animation: Option<Animation>,
                pub requirements: Requirements,

                pub rounds: Rounds,
                pub caliber: Option<super::super::common::weapons::Caliber>,

                pub perk: Option<super::super::common::critter::Perk>,
                pub connections: Connections,
            }
        }

        pub mod ammo {
            pub mod adjustments {
                pub struct Armor {
                    pub class: u32,
                    pub resistance: u32,
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
                pub power_item_id: u32,
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
            pub ai_packet_id: u32,
        }

        pub struct Instance {
            pub team: u32,

            pub murder: murder::Result,
            pub damage: super::common::combat::damage::Type,

            pub body: super::common::critter::body::Type,
            pub sprite: Option<super::common::sprite::Instance>,
            pub script: Option<super::common::script::Instance>,

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
            pub _sounds_ids: u32,
        }

        pub struct Instance {
            pub r#type: Type,

            pub light: super::common::world::Light,
            pub script: super::common::script::Instance,
            pub material: super::common::world::Material,

            pub actions: std::collections::HashSet<super::common::actions::Instance>,
            pub connections: Connections,
        }

        pub mod door {
            use std::collections::HashSet;

            pub enum Flag {
                Passable
            }

            pub struct Instance {
                pub flags: HashSet<Flag>
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
        pub struct Instance {
            pub light: super::common::world::Light,
            pub script: super::common::script::Instance,
            pub material: super::common::world::Material,

            pub actions: std::collections::HashSet<super::common::actions::Instance>,
        }
    }

    pub mod tile {
        pub struct Instance {
            pub material: super::common::world::Material,
        }
    }
}
