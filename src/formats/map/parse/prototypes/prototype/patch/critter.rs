use std::collections::HashMap;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::formats::pro::object::common::critter::Statistic;
use crate::formats::pro::object::critter::{Connections, Patch, Statistics};

pub(crate) fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    let _damage_last_turn = source.read_u32::<BigEndian>()?;
    let _combat_state = source.read_u32::<BigEndian>()?;
    let _action_points = source.read_u32::<BigEndian>()?;
    let _damage_flags = source.read_u32::<BigEndian>()?;

    let ai_packet_id = u32::try_from(source.read_i32::<BigEndian>()?).unwrap_or(0);
    let team_id = source.read_u32::<BigEndian>()?;

    let _who_hit_me = source.read_u32::<BigEndian>()?;

    let health = source.read_u32::<BigEndian>()? as i32;
    let radiation = source.read_u32::<BigEndian>()? as i32;
    let poison = source.read_u32::<BigEndian>()? as i32;

    Ok(Patch {
        team: team_id,
        statistics: Statistics {
            basic: HashMap::from([
                (Statistic::CurrentHitPoints, health),
                (Statistic::CurrentPoisonLevel, poison),
                (Statistic::CurrentRadiationLevel, radiation),
            ]),
            bonuses: Default::default(),
        },
        connections: Connections { ai_packet_id },
    })
}