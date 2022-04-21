use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;
use std::time::Duration;

use super::*;
use super::super::super::common::types::ScaledValue;

mod r#type;
mod flags;
mod id;

pub mod errors {
    #[derive(Debug)]
    pub enum Format {
        Type,
        Data,
        Flags,
        Consistency,
    }

    #[derive(Debug)]
    pub enum Error {
        Read(std::io::Error),
        Format(Format),
        Source,
    }
}

pub fn prototype<S: Read + Seek>(source: &mut S) -> Result<Prototype, errors::Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(errors::Error::Read(error));
    }

    let mut id_bytes = [0u8; 4];
    match source.read_exact(&mut id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let (type_id, object_id) = match id::instance(id_bytes) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let mut text_id_bytes = [0u8; 4];
    match source.read_exact(&mut text_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let text_id = u32::from_be_bytes(text_id_bytes);

    let mut sprite_id_bytes = [0u8; 4];
    match source.read_exact(&mut sprite_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut light_radius_bytes = [0u8; 4];
    match source.read_exact(&mut light_radius_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let light_radius = light_radius_bytes[3];

    let mut light_intensity_bytes = [0u8; 4];
    match source.read_exact(&mut light_intensity_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let light_intensity =
        u16::from_be_bytes(match &light_intensity_bytes[2..4].try_into() {
            Ok(value) => *value,
            Err(_) => return Err(errors::Error::Source),
        });

    let flags = match flags::instance(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let r#type = match r#type::instance(source, type_id) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    Ok(Prototype {
        id: object_id,
        meta: meta::Info {
            light: meta::info::Light {
                distance: ScaledValue {
                    value: light_radius,
                    scale: Range { start: 0, end: 8 },
                },
                intensity: ScaledValue {
                    value: light_intensity,
                    scale: Range { start: 0, end: u16::MAX },
                },
            },
            flags,
            sprite: match object::common::sprite::Reference::try_from(sprite_id_bytes) {
                Ok(value) => value,
                Err(error) => return Err(error)
            },
            connections: meta::info::Connections { description_id: text_id },
        },
        r#type,
    })
}

impl TryFrom<u32> for object::common::world::Material {
    type Error = errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Glass),
            1 => Ok(Self::Metal),
            2 => Ok(Self::Plastic),
            3 => Ok(Self::Wood),
            4 => Ok(Self::Dirt),
            5 => Ok(Self::Stone),
            6 => Ok(Self::Cement),
            7 => Ok(Self::Leather),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<u8> for object::item::weapon::attack::Mode {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Punch),
            2 => Ok(Self::Kick),
            3 => Ok(Self::Swing),
            4 => Ok(Self::Thrust),
            5 => Ok(Self::Throw),
            6 => Ok(Self::FireSingle),
            7 => Ok(Self::FireBurst),
            8 => Ok(Self::Flame),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<i32> for object::common::critter::Perk {
    type Error = errors::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Awareness),
            2 => Ok(Self::BonusHtHAttacks),
            3 => Ok(Self::BonusHtHDamage),
            4 => Ok(Self::BonusMove),
            5 => Ok(Self::BonusRangedDamage),
            6 => Ok(Self::BonusRateOfFire),
            7 => Ok(Self::EarlierSequence),
            8 => Ok(Self::FasterHealing),
            9 => Ok(Self::MoreCriticals),
            10 => Ok(Self::NightVision),
            11 => Ok(Self::Presence),
            12 => Ok(Self::RadResistance),
            13 => Ok(Self::Toughness),
            14 => Ok(Self::StrongBack),
            15 => Ok(Self::Sharpshooter),
            16 => Ok(Self::SilentRunning),
            17 => Ok(Self::Survivalist),
            18 => Ok(Self::MasterTrader),
            19 => Ok(Self::Educated),
            20 => Ok(Self::Healer),
            21 => Ok(Self::FortuneFinder),
            22 => Ok(Self::BetterCriticals),
            23 => Ok(Self::Empathy),
            24 => Ok(Self::Slayer),
            25 => Ok(Self::Sniper),
            26 => Ok(Self::SilentDeath),
            27 => Ok(Self::ActionBoy),
            28 => Ok(Self::MentalBlock),
            29 => Ok(Self::Lifegiver),
            30 => Ok(Self::Dodger),
            31 => Ok(Self::Snakeater),
            32 => Ok(Self::MrFixit),
            33 => Ok(Self::Medic),
            34 => Ok(Self::MasterThief),
            35 => Ok(Self::Speaker),
            36 => Ok(Self::HeaveHo),
            37 => Ok(Self::FriendlyFoe),
            38 => Ok(Self::Pickpocket),
            39 => Ok(Self::Ghost),
            40 => Ok(Self::CultOfPersonality),
            41 => Ok(Self::Scrounger),
            42 => Ok(Self::Explorer),
            43 => Ok(Self::FlowerChild),
            44 => Ok(Self::Pathfinder),
            45 => Ok(Self::AnimalFriend),
            46 => Ok(Self::Scout),
            47 => Ok(Self::MysteriousStranger),
            48 => Ok(Self::Ranger),
            49 => Ok(Self::QuickPockets),
            50 => Ok(Self::SmoothTalker),
            51 => Ok(Self::SwiftLearner),
            52 => Ok(Self::Tag),
            53 => Ok(Self::Mutate),
            54 => Ok(Self::NukaColaAddiction),
            55 => Ok(Self::BuffoutAddiction),
            56 => Ok(Self::MentatsAddiction),
            57 => Ok(Self::PsychoAddiction),
            58 => Ok(Self::RadawayAddiction),
            59 => Ok(Self::WeaponLongRange),
            60 => Ok(Self::WeaponAccurate),
            61 => Ok(Self::WeaponPenetrate),
            62 => Ok(Self::WeaponKnockback),
            63 => Ok(Self::PoweredArmor),
            64 => Ok(Self::CombatArmor),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<i32> for object::common::critter::Statistic {
    type Error = errors::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Strength),
            2 => Ok(Self::Perception),
            3 => Ok(Self::Endurance),
            4 => Ok(Self::Charisma),
            5 => Ok(Self::Intelligence),
            6 => Ok(Self::Agility),
            7 => Ok(Self::Luck),
            8 => Ok(Self::MaximumHitPoints),
            9 => Ok(Self::MaximumActionPoints),
            10 => Ok(Self::ArmorClass),
            11 => Ok(Self::UnarmedDamage),
            12 => Ok(Self::MeleeDamage),
            13 => Ok(Self::CarryWeight),
            14 => Ok(Self::Sequence),
            15 => Ok(Self::HealingRate),
            16 => Ok(Self::CriticalChance),
            17 => Ok(Self::BetterCriticals),
            18 => Ok(Self::DamageThreshold),
            19 => Ok(Self::DamageThresholdLaser),
            20 => Ok(Self::DamageThresholdFire),
            21 => Ok(Self::DamageThresholdPlasma),
            22 => Ok(Self::DamageThresholdElectrical),
            23 => Ok(Self::DamageThresholdEMP),
            24 => Ok(Self::DamageThresholdExplosion),
            25 => Ok(Self::DamageResistance),
            26 => Ok(Self::DamageResistanceLaser),
            27 => Ok(Self::DamageResistanceFire),
            28 => Ok(Self::DamageResistancePlasma),
            29 => Ok(Self::DamageResistanceElectrical),
            30 => Ok(Self::DamageResistanceEMP),
            31 => Ok(Self::DamageResistanceExplosion),
            32 => Ok(Self::RadiationResistance),
            33 => Ok(Self::PoisonResistance),
            34 => Ok(Self::Age),
            35 => Ok(Self::Gender),
            36 => Ok(Self::CurrentHitPoints),
            37 => Ok(Self::CurrentPoisonLevel),
            38 => Ok(Self::CurrentRadiationLevel),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<u32> for object::item::weapon::Animation {
    type Error = errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Knife),
            0x02 => Ok(Self::Club),
            0x03 => Ok(Self::Sledgehammer),
            0x04 => Ok(Self::Spear),
            0x05 => Ok(Self::Pistol),
            0x06 => Ok(Self::SubmachineGun),
            0x07 => Ok(Self::Rifle),
            0x08 => Ok(Self::BigGun),
            0x09 => Ok(Self::Minigun),
            0x0A => Ok(Self::RocketLauncher),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<u8> for object::common::combat::damage::Type {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Laser),
            2 => Ok(Self::Fire),
            3 => Ok(Self::Plasma),
            4 => Ok(Self::Electrical),
            5 => Ok(Self::Emp),
            6 => Ok(Self::Explosive),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<u32> for object::common::weapons::Caliber {
    type Error = errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Rocket),
            2 => Ok(Self::FlamethrowerFuel),
            3 => Ok(Self::CEnergyCell),
            4 => Ok(Self::DEnergyCell),
            5 => Ok(Self::Remington223),
            6 => Ok(Self::FiveMillimeter),
            7 => Ok(Self::SnW40),
            8 => Ok(Self::TenMillimiter),
            9 => Ok(Self::Magnum44),
            10 => Ok(Self::FourteenMillimeter),
            11 => Ok(Self::TwelveGauge),
            12 => Ok(Self::NineMillimeter),
            13 => Ok(Self::Bb),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<[u8; 4]> for object::common::sprite::Reference {
    type Error = errors::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let (type_id, id) = match id::instance(value) {
            Ok(value) => value,
            Err(error) => return Err(error)
        };

        let r#type = match type_id {
            0x00 => object::common::sprite::Type::Item,
            0x01 => object::common::sprite::Type::Critter,
            0x02 => object::common::sprite::Type::Scenery,
            0x03 => object::common::sprite::Type::Wall,
            0x04 => object::common::sprite::Type::Tile,
            0x05 => object::common::sprite::Type::Background,
            0x06 => object::common::sprite::Type::Interface,
            0x07 => object::common::sprite::Type::Inventory,
            _ => return Err(errors::Error::Format(errors::Format::Data)),
        };

        return Ok(Self {
            id,
            r#type,
        });
    }
}

impl TryFrom<[u8; 4]> for object::common::script::Reference {
    type Error = errors::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let (type_id, id) = match id::instance(value) {
            Ok(value) => value,
            Err(error) => return Err(error)
        };

        let r#type = match type_id {
            0x01 => object::common::script::Type::Spatial,
            0x02 => object::common::script::Type::Item,
            0x03 => object::common::script::Type::Scenery,
            0x04 => object::common::script::Type::Critter,
            _ => return Err(errors::Error::Format(errors::Format::Data)),
        };

        return Ok(Self {
            id,
            r#type,
        });
    }
}

trait TryFromOptional<T>: TryFrom<T> where T: Eq {
    fn try_from_optional(value: T, none: T) -> Result<Option<Self>, Self::Error> {
        Ok(if value == none { None } else {
            Some(match Self::try_from(value) {
                Ok(value) => value,
                Err(error) => return Err(error)
            })
        })
    }
}

impl<V, T> TryFromOptional<T> for V where V: TryFrom<T>, T: Eq {}