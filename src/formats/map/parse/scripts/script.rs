use crate::common::types::geometry::{Coordinate, Scaled};
use crate::common::types::models;
use crate::common::types::models::script::Kind::{Critter, Item, Spatial, System, Timed};
use crate::common::types::space::Elevation;

use super::super::*;

pub fn instance<S: Read + Seek>(source: &mut S, type_raw: u8) -> Result<blueprint::script::Instance, errors::Error> {
    source.seek(SeekFrom::Current(2))?;
    let id = source.read_u16::<BigEndian>()?;

    source.seek(SeekFrom::Current(4))?;

    let mut timed_inners: Option<blueprint::script::time::Instance> = None;
    let mut spatial_inners: Option<blueprint::script::spatial::Instance> = None;

    match type_raw {
        1 => {
            const LEVELS_SCALE: std::ops::Range<u8> = 0u8..3;
            let elevation =
                match source.read_u16::<BigEndian>()? {
                    0x0000 => Elevation { level: Scaled { value: 0u8, scale: LEVELS_SCALE } },
                    0x2000 => Elevation { level: Scaled { value: 1u8, scale: LEVELS_SCALE } },
                    0x4000 => Elevation { level: Scaled { value: 2u8, scale: LEVELS_SCALE } },
                    _ => return Err(errors::Error::Format)
                };

            let position = Coordinate::try_from(source.read_u16::<BigEndian>()? as u32)?;
            let distance = source.read_u32::<BigEndian>()? as u16;

            spatial_inners = Some(blueprint::script::spatial::Instance {
                position,
                distance,
                elevation,
            })
        }
        2 => {
            timed_inners = Some(blueprint::script::time::Instance {
                duration: std::time::Duration::new(source.read_u32::<BigEndian>()? as u64, 0)
            });
        }
        _ => {}
    }

    let _flags = source.read_u32::<BigEndian>()? as u16;
    let program_id = source.read_i32::<BigEndian>()?;

    source.seek(SeekFrom::Current(4))?;

    let object_id = source.read_i32::<BigEndian>()?;

    let local_vars_offset = source.read_i32::<BigEndian>()?;
    let local_vars_count = source.read_i32::<BigEndian>()?;

    let _return_value = source.read_i32::<BigEndian>()?;
    let _actions = source.read_i32::<BigEndian>()?;
    let _exit_parameters = source.read_i32::<BigEndian>()?;
    let _actions_count = source.read_i32::<BigEndian>()?;
    let _script_overrides = source.read_i32::<BigEndian>()?;

    source.seek(SeekFrom::Current(4))?;

    let _how_much = source.read_i32::<BigEndian>()?;

    source.seek(SeekFrom::Current(4))?;

    Ok(blueprint::script::Instance {
        id,
        kind: match models::script::Type::try_from(u32::try_from(type_raw).map_err(|_| errors::Error::Format)?)? {
            System(_) => System(()),
            Spatial(_) => Spatial(spatial_inners.ok_or(errors::Error::Format)?),
            Timed(_) => Timed(timed_inners.ok_or(errors::Error::Format)?),
            Item(_) => Item(()),
            Critter(_) => Critter(()),
        },
        variables: if local_vars_offset > -1 && local_vars_count > 0 {
            Some(blueprint::script::Variables {
                offset: local_vars_offset as u32,
                count: local_vars_count as u32,
            })
        } else { None },
        connections: blueprint::script::Connections {
            program_id: u32::try_from(program_id).ok(),
            object_id: u32::try_from(object_id).ok(),
        },
    })
}

pub fn skip<S: Read + Seek>(source: &mut S) -> std::io::Result<u64> {
    let read_type = source.read_u8()?;
    source.seek(SeekFrom::Current(3))?;

    source.seek(SeekFrom::Current(4 * (15 + match read_type {
        1 => 2,
        2 => 0,
        _ => 0
    })))
}