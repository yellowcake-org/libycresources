use crate::common::types::geometry::{Coordinate, Scaled};
use crate::formats::map::blueprint::script::Type::{Critter, Item, Spatial, System, Time};
use crate::formats::map::common::Elevation;

use super::super::*;

pub fn instance<S: Read + Seek>(source: &mut S, type_raw: u32) -> Result<blueprint::script::Instance, errors::Error> {
    let mut id_bytes = [0u8; 4];
    source.read_exact(&mut id_bytes)?;

    let id = u16::from_be_bytes(id_bytes[2..4].try_into().map_err(|_| errors::Error::Format)?);
    source.seek(SeekFrom::Current(4))?;

    let mut timed_inners: Option<blueprint::script::time::Instance> = None;
    let mut spatial_inners: Option<blueprint::script::spatial::Instance> = None;

    match type_raw {
        1 => {
            let mut elevation_n_tile_bytes = [0u8; 4];
            source.read_exact(&mut elevation_n_tile_bytes)?;

            const LEVELS_SCALE: std::ops::Range<u8> = 0u8..3;
            let elevation =
                match u16::from_be_bytes(elevation_n_tile_bytes[0..2].try_into()
                    .map_err(|_| errors::Error::Format)?
                ) {
                    0x0000 => Elevation { level: Scaled { value: 0u8, scale: LEVELS_SCALE } },
                    0x2000 => Elevation { level: Scaled { value: 1u8, scale: LEVELS_SCALE } },
                    0x4000 => Elevation { level: Scaled { value: 2u8, scale: LEVELS_SCALE } },
                    _ => return Err(errors::Error::Format)
                };

            let position = Coordinate::try_from(source.read_u32::<BigEndian>()?)?;
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
    let program_id = source.read_u32::<BigEndian>()?;

    source.seek(SeekFrom::Current(4))?;

    let object_id = source.read_i32::<BigEndian>()?;

    let lvars_offset = source.read_i32::<BigEndian>()?;
    let lvars_count = source.read_i32::<BigEndian>()?;

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
        r#type: match type_raw {
            0 => System,
            1 => Spatial(match spatial_inners {
                Some(value) => value,
                None => return Err(errors::Error::Format)
            }),
            2 => Time(match timed_inners {
                Some(value) => value,
                None => return Err(errors::Error::Format)
            }),
            3 => Item,
            4 => Critter,
            _ => return Err(errors::Error::Format)
        },
        variables: if lvars_offset > -1 && lvars_count > 0 {
            Some(blueprint::script::Variables {
                offset: lvars_offset as u32,
                count: lvars_count as u32,
            })
        } else { None },
        connections: blueprint::script::Connections {
            program_id,
            object_id: if object_id > -1 { Some(object_id as u32) } else { None },
        },
    })
}

pub fn skip<S: Seek>(source: &mut S) -> std::io::Result<u64> {
    source.seek(SeekFrom::Current(4 * 16))
}