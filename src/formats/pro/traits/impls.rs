use crate::common::types::errors;
use crate::common::types::geometry::{Coordinate, Elevation, Scaled};

use super::super::object;

impl TryFrom<u32> for object::common::critter::body::Type {
    type Error = errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Biped),
            1 => Ok(Self::Quadruped),
            2 => Ok(Self::Robotic),
            _ => Err(errors::Error::Format)
        }
    }
}

impl TryFrom<u8> for object::common::critter::Gender {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Male),
            1 => Ok(Self::Female),
            _ => Err(errors::Error::Format)
        }
    }
}

impl TryFrom<u32> for object::critter::murder::Type {
    type Error = errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Men),
            1 => Ok(Self::Women),
            2 => Ok(Self::Children),
            3 => Ok(Self::SuperMutants),
            4 => Ok(Self::Ghouls),
            5 => Ok(Self::Brahmin),
            6 => Ok(Self::Radscorpions),
            7 => Ok(Self::Rats),
            8 => Ok(Self::Floaters),
            9 => Ok(Self::Centaurs),
            10 => Ok(Self::Robots),
            11 => Ok(Self::Dogs),
            12 => Ok(Self::Manti),
            13 => Ok(Self::DeathClaws),
            14 => Ok(Self::Plants),
            15 => Ok(Self::Geckos),
            16 => Ok(Self::Aliens),
            17 => Ok(Self::GiantAnts),
            18 => Ok(Self::GiantAnts),
            _ => Err(errors::Error::Format)
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
            _ => Err(errors::Error::Format)
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
            _ => Err(errors::Error::Format)
        }
    }
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
            _ => Err(errors::Error::Format)
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
            8 => Ok(Self::TenMillimeter),
            9 => Ok(Self::Magnum44),
            10 => Ok(Self::FourteenMillimeter),
            11 => Ok(Self::TwelveGauge),
            12 => Ok(Self::NineMillimeter),
            13 => Ok(Self::Bb),
            other => Ok(Self::Unknown(other))
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
            _ => Err(errors::Error::Format)
        }
    }
}

impl TryFrom<i32> for object::common::critter::Perk {
    type Error = errors::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown(0)),
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
            _ => Err(errors::Error::Format)
        }
    }
}

impl TryFrom<i32> for object::common::critter::Statistic {
    type Error = errors::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Strength),
            1 => Ok(Self::Perception),
            2 => Ok(Self::Endurance),
            3 => Ok(Self::Charisma),
            4 => Ok(Self::Intelligence),
            5 => Ok(Self::Agility),
            6 => Ok(Self::Luck),
            7 => Ok(Self::MaximumHitPoints),
            8 => Ok(Self::MaximumActionPoints),
            9 => Ok(Self::ArmorClass),
            10 => Ok(Self::UnarmedDamage),
            11 => Ok(Self::MeleeDamage),
            12 => Ok(Self::CarryWeight),
            13 => Ok(Self::Sequence),
            14 => Ok(Self::HealingRate),
            15 => Ok(Self::CriticalChance),
            16 => Ok(Self::BetterCriticals),
            17 => Ok(Self::DamageThreshold),
            18 => Ok(Self::DamageThresholdLaser),
            19 => Ok(Self::DamageThresholdFire),
            20 => Ok(Self::DamageThresholdPlasma),
            21 => Ok(Self::DamageThresholdElectrical),
            22 => Ok(Self::DamageThresholdEMP),
            23 => Ok(Self::DamageThresholdExplosive),
            24 => Ok(Self::DamageResistance),
            25 => Ok(Self::DamageResistanceLaser),
            26 => Ok(Self::DamageResistanceFire),
            27 => Ok(Self::DamageResistancePlasma),
            28 => Ok(Self::DamageResistanceElectrical),
            29 => Ok(Self::DamageResistanceEMP),
            30 => Ok(Self::DamageResistanceExplosive),
            31 => Ok(Self::RadiationResistance),
            32 => Ok(Self::PoisonResistance),
            33 => Ok(Self::Age),
            34 => Ok(Self::Gender),
            35 => Ok(Self::CurrentHitPoints),
            36 => Ok(Self::CurrentPoisonLevel),
            37 => Ok(Self::CurrentRadiationLevel),
            _ => Err(errors::Error::Format)
        }
    }
}

impl TryFrom<&[u8; 4]> for object::common::map::Destination {
    type Error = errors::Error;

    fn try_from(value: &[u8; 4]) -> Result<Self, Self::Error> {
        let floor: Elevation = match value[0] & 0xFF {
            0xF0 => Elevation { level: Scaled { value: 0, scale: u8::MIN..3 } },
            0xF2 => Elevation { level: Scaled { value: 1, scale: u8::MIN..3 } },
            0xF4 => Elevation { level: Scaled { value: 2, scale: u8::MIN..3 } },
            _ => return Err(errors::Error::Format),
        };

        Ok(Self {
            elevation: floor,
            position: Coordinate::try_from(u32::from_be_bytes([0u8, value[1], value[2], value[3]]))?,
        })
    }
}

impl TryFrom<i32> for object::common::map::Map {
    type Error = errors::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::World),
            value => Ok(
                Self::Local(match u32::try_from(value) {
                    Ok(value) => value,
                    Err(_) => return Err(errors::Error::Format)
                })
            )
        }
    }
}