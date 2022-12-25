use std::io::ErrorKind;

use object::critter::Instance;

use crate::common::traits::TryFromOptional;

use super::super::*;

mod flags;
mod skills;
mod statistics;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let mut flags_bytes = [0u8; 4];
    source.read_exact(&mut flags_bytes)?;

    let script =
        Identifier::try_from_optional(source.read_u32::<BigEndian>()?, 0xFF_FF_FF_FF)?;

    let head =
        Identifier::try_from_optional(source.read_u32::<BigEndian>()?, 0xFF_FF_FF_FF)?;

    let ai_packet_id = source.read_u32::<BigEndian>()?;
    let team = source.read_u32::<BigEndian>()?;

    let flags = flags::instance(source)?;

    let basic = statistics::map(source)?;
    let bonuses = statistics::map(source)?;

    let skills = skills::map(source)?;
    let body = object::common::critter::body::Type::try_from(source.read_u32::<BigEndian>()?)?;

    let kill_reward = source.read_u32::<BigEndian>()?;
    let kill_type = object::critter::murder::Type::try_from(source.read_u32::<BigEndian>()?)?;

    let damage_type = {
        match source.read_u32::<BigEndian>() {
            Ok(v) => { Some(object::common::combat::damage::Type::try_from(v as u8)?) }
            Err(error) => {
                match error.kind() {
                    ErrorKind::UnexpectedEof => None, // file is from Fallout™ 1
                    _ => return Err(errors::Error::IO(error))
                }
            }
        }
    };

    Ok(Instance {
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
