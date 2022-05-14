use std::io::ErrorKind;

use super::super::*;
use super::super::super::traits::TryFromOptional;

mod flags;
mod skills;
mod statistics;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::critter::Instance, errors::Error> {
    let mut flags_bytes = [0u8; 4];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut script_id_bytes = [0u8; 4];
    match source.read_exact(&mut script_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let script =
        match object::common::script::Reference::
        try_from_optional(script_id_bytes, [0xFF, 0xFF, 0xFF, 0xFF]) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        };

    let mut head_id_bytes = [0u8; 4];
    match source.read_exact(&mut head_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let head = match object::common::sprite::Reference::
    try_from_optional(head_id_bytes, [0xFF, 0xFF, 0xFF, 0xFF]) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
    };

    let mut ai_packet_id_bytes = [0u8; 4];
    match source.read_exact(&mut ai_packet_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ai_packet_id = u32::from_be_bytes(ai_packet_id_bytes);

    let mut team_bytes = [0u8; 4];
    match source.read_exact(&mut team_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let team = u32::from_be_bytes(team_bytes);

    let flags = match flags::instance(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let basic = match statistics::map(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let bonuses = match statistics::map(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let skills = match skills::map(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let mut body_bytes = [0u8; 4];
    match source.read_exact(&mut body_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let body_raw = u32::from_be_bytes(body_bytes);
    let body = match object::common::critter::body::Type::try_from(body_raw) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let mut kill_reward_bytes = [0u8; 4];
    match source.read_exact(&mut kill_reward_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let kill_reward = u32::from_be_bytes(kill_reward_bytes);

    let mut kill_type_bytes = [0u8; 4];
    match source.read_exact(&mut kill_type_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let kill_type_raw = u32::from_be_bytes(kill_type_bytes);
    let kill_type = match object::critter::murder::Type::try_from(kill_type_raw) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
    };

    let mut damage_type = None;

    let mut damage_type_bytes = [0u8; 4];
    match source.read_exact(&mut damage_type_bytes) {
        Err(error) => {
            match error.kind() {
                ErrorKind::UnexpectedEof => {}
                _ => return Err(errors::Error::Source)
            }
        }
        Ok(_) => {
            let damage_type_raw = u32::from_be_bytes(damage_type_bytes);
            damage_type = Some(match object::common::combat::damage::Type::try_from(damage_type_raw as u8) {
                Ok(value) => value,
                Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
            });
        }
    };

    Ok(object::critter::Instance {
        team,
        murder: object::critter::murder::Result {
            r#type: kill_type,
            experience: kill_reward,
        },
        damage: damage_type,
        body,
        head,
        script,
        flags,
        skills,
        statistics: object::critter::Statistics { basic, bonuses },
        connections: object::critter::Connections { ai_packet_id },
    })
}
