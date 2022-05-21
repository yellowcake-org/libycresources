use std::collections::HashMap;

use crate::formats::pro::object::common::critter::Statistic;
use crate::formats::pro::parse::*;

pub(crate) fn map<S: Read>(source: &mut S) ->
Result<HashMap<Statistic, i32>, errors::Error> {
    let mut statistics = HashMap::new();

    fn consume<S: Read>(statistic: Statistic,
                        source: &mut S,
                        into: &mut HashMap<Statistic, i32>) ->
                        Result<(), errors::Error> {
        into.insert(statistic, source.read_i32::<BigEndian>()?);
        Ok(())
    }

    // S.P.E.C.I.A.L.

    consume(Statistic::Strength, source, &mut statistics)?;
    consume(Statistic::Perception, source, &mut statistics)?;
    consume(Statistic::Endurance, source, &mut statistics)?;
    consume(Statistic::Charisma, source, &mut statistics)?;
    consume(Statistic::Intelligence, source, &mut statistics)?;
    consume(Statistic::Agility, source, &mut statistics)?;
    consume(Statistic::Luck, source, &mut statistics)?;

    // Other

    consume(Statistic::MaximumHitPoints, source, &mut statistics)?;
    consume(Statistic::MaximumActionPoints, source, &mut statistics)?;
    consume(Statistic::ArmorClass, source, &mut statistics)?;
    consume(Statistic::UnarmedDamage, source, &mut statistics)?;
    consume(Statistic::MeleeDamage, source, &mut statistics)?;
    consume(Statistic::CarryWeight, source, &mut statistics)?;
    consume(Statistic::Sequence, source, &mut statistics)?;
    consume(Statistic::HealingRate, source, &mut statistics)?;
    consume(Statistic::CriticalChance, source, &mut statistics)?;
    consume(Statistic::BetterCriticals, source, &mut statistics)?;

    // Damage Threshold

    consume(Statistic::DamageThreshold, source, &mut statistics)?;
    consume(Statistic::DamageThresholdLaser, source, &mut statistics)?;
    consume(Statistic::DamageThresholdFire, source, &mut statistics)?;
    consume(Statistic::DamageThresholdPlasma, source, &mut statistics)?;
    consume(Statistic::DamageThresholdElectrical, source, &mut statistics)?;
    consume(Statistic::DamageThresholdEMP, source, &mut statistics)?;
    consume(Statistic::DamageThresholdExplosive, source, &mut statistics)?;

    // Damage Resistance

    consume(Statistic::DamageResistance, source, &mut statistics)?;
    consume(Statistic::DamageResistanceLaser, source, &mut statistics)?;
    consume(Statistic::DamageResistanceFire, source, &mut statistics)?;
    consume(Statistic::DamageResistancePlasma, source, &mut statistics)?;
    consume(Statistic::DamageResistanceElectrical, source, &mut statistics)?;
    consume(Statistic::DamageResistanceEMP, source, &mut statistics)?;
    consume(Statistic::DamageResistanceExplosive, source, &mut statistics)?;
    consume(Statistic::RadiationResistance, source, &mut statistics)?;
    consume(Statistic::PoisonResistance, source, &mut statistics)?;

    // Age & Gender

    consume(Statistic::Age, source, &mut statistics)?;
    consume(Statistic::Gender, source, &mut statistics)?;

    Ok(statistics)
}