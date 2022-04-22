use std::collections::HashSet;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<HashSet<object::critter::Flag>, errors::Error> {
    let mut flags_bytes = [0u8; 4];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut flags: HashSet<object::critter::Flag> = HashSet::new();

    if (flags_bytes[3] & 0x02) == 0x02 &&
        !flags.insert(object::critter::Flag::BarterAvailable) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[3] & 0x20) == 0x20 &&
        !flags.insert(object::critter::Flag::NoSteal) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[3] & 0x40) == 0x40 &&
        !flags.insert(object::critter::Flag::NoDrop) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[3] & 0x80) == 0x80 &&
        !flags.insert(object::critter::Flag::NoLimbsLoose) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[2] & 0x01) == 0x01 &&
        !flags.insert(object::critter::Flag::NoCorpseDisappear) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[2] & 0x02) == 0x02 &&
        !flags.insert(object::critter::Flag::NoAutoHeal) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[2] & 0x04) == 0x04 &&
        !flags.insert(object::critter::Flag::Invulnerable) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[2] & 0x08) == 0x08 &&
        !flags.insert(object::critter::Flag::NoCorpse) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[2] & 0x10) == 0x10 &&
        !flags.insert(object::critter::Flag::SpecialDeath) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[2] & 0x20) == 0x20 &&
        !flags.insert(object::critter::Flag::RangedMelee) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    if (flags_bytes[2] & 0x40) == 0x40 &&
        !flags.insert(object::critter::Flag::NoKnockDown) {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    Ok(flags)
}