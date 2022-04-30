use std::collections::HashMap;

use crate::formats::pro::object::common::critter::Statistic;
use crate::formats::pro::parse::*;

pub(crate) fn map<S: Read>(source: &mut S) ->
Result<HashMap<object::common::critter::Statistic, u32>, errors::Error> {
    let mut statistics = HashMap::new();

    fn consume<S: Read>(statistic: object::common::critter::Statistic,
                        source: &mut S,
                        into: &mut HashMap<object::common::critter::Statistic, u32>) ->
                        Result<(), errors::Error> {
        let mut value_bytes = [0u8; 4];
        match source.read_exact(&mut value_bytes) {
            Err(error) => return Err(errors::Error::Read(error)),
            Ok(value) => value,
        };

        let value = u32::from_be_bytes(value_bytes);
        into.insert(statistic, value);

        Ok(())
    }

    // S.P.E.C.I.A.L.

    if let Err(error) = consume(Statistic::Strength, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::Perception, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::Endurance, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::Charisma, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::Intelligence, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::Agility, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::Luck, source, &mut statistics) {
        return Err(error);
    }

    // Other

    if let Err(error) = consume(Statistic::MaximumHitPoints, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::MaximumActionPoints, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::ArmorClass, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::UnarmedDamage, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::MeleeDamage, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::CarryWeight, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::Sequence, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::HealingRate, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::CriticalChance, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::BetterCriticals, source, &mut statistics) {
        return Err(error);
    }

    // Damage Threshold

    if let Err(error) = consume(Statistic::DamageThreshold, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageThresholdLaser, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageThresholdFire, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageThresholdPlasma, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageThresholdElectrical, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageThresholdEMP, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageThresholdExplosive, source, &mut statistics) {
        return Err(error);
    }

    // Damage Resistance

    if let Err(error) = consume(Statistic::DamageResistance, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageResistanceLaser, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageResistanceFire, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageResistancePlasma, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageResistanceElectrical, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageResistanceEMP, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::DamageResistanceExplosive, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::RadiationResistance, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::PoisonResistance, source, &mut statistics) {
        return Err(error);
    }

    // Age & Gender

    if let Err(error) = consume(Statistic::Age, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::Gender, source, &mut statistics) {
        return Err(error);
    }

    Ok(statistics)
}