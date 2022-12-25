use std::collections::HashMap;

use object::common::critter::Skill;

use super::super::super::*;

pub(crate) fn map<S: Read>(source: &mut S) -> Result<HashMap<Skill, u32>, errors::Error> {
    let mut skills = HashMap::new();

    fn consume<S: Read>(skill: Skill,
                        source: &mut S,
                        into: &mut HashMap<Skill, u32>) ->
                        Result<(), errors::Error> {
        into.insert(skill, source.read_u32::<BigEndian>()?);
        Ok(())
    }

    consume(Skill::SmallGuns, source, &mut skills)?;
    consume(Skill::BigGuns, source, &mut skills)?;
    consume(Skill::EnergyWeapons, source, &mut skills)?;
    consume(Skill::Unarmed, source, &mut skills)?;
    consume(Skill::MeleeWeapons, source, &mut skills)?;
    consume(Skill::Throwing, source, &mut skills)?;
    consume(Skill::FirstAid, source, &mut skills)?;
    consume(Skill::Doctor, source, &mut skills)?;
    consume(Skill::Sneak, source, &mut skills)?;
    consume(Skill::Lockpick, source, &mut skills)?;
    consume(Skill::Steal, source, &mut skills)?;
    consume(Skill::Traps, source, &mut skills)?;
    consume(Skill::Science, source, &mut skills)?;
    consume(Skill::Repair, source, &mut skills)?;
    consume(Skill::Speech, source, &mut skills)?;
    consume(Skill::Barter, source, &mut skills)?;
    consume(Skill::Gambling, source, &mut skills)?;
    consume(Skill::Outdoorsman, source, &mut skills)?;

    Ok(skills)
}