use std::collections::HashMap;

use object::common::critter::Skill;

use super::super::super::*;

pub(crate) fn map<S: Read>(source: &mut S) -> Result<HashMap<Skill, u32>, errors::Error> {
    let mut skills = HashMap::new();

    fn consume<S: Read>(skill: object::common::critter::Skill,
                        source: &mut S,
                        into: &mut HashMap<object::common::critter::Skill, u32>) ->
                        Result<(), errors::Error> {
        let mut value_bytes = [0u8; 4];
        match source.read_exact(&mut value_bytes) {
            Err(error) => return Err(errors::Error::Read(error)),
            Ok(value) => value,
        };

        let value = u32::from_be_bytes(value_bytes);
        into.insert(skill, value);

        Ok(())
    }

    if let Err(error) = consume(Skill::SmallGuns, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::BigGuns, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::EnergyWeapons, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Unarmed, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::MeleeWeapons, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Throwing, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::FirstAid, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Doctor, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Sneak, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Lockpick, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Steal, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Traps, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Science, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Repair, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Speech, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Barter, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Gambling, source, &mut skills) {
        return Err(error);
    }

    if let Err(error) = consume(Skill::Outdoorsman, source, &mut skills) {
        return Err(error);
    }

    Ok(skills)
}