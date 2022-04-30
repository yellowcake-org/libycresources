use std::collections::HashMap;

use crate::formats::pro::object::common::critter::Statistic;

use super::super::super::super::*;

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

    if let Err(error) = consume(Statistic::HitPoints, source, &mut statistics) {
        return Err(error);
    }

    if let Err(error) = consume(Statistic::ActionPoints, source, &mut statistics) {
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

    Ok(statistics)
}