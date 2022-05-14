use crate::common::types::Scaled;
use crate::formats::map::common::{Coordinate, Elevation};
use crate::formats::map::state::blueprint::Type::{Critter, Item, Spatial, System, Time};

use super::super::*;

pub fn instance<S: Read + Seek>(source: &mut S, type_raw: u32) -> Result<state::blueprint::Instance, errors::Error> {
    let mut id_bytes = [0u8; 4];
    match source.read_exact(&mut id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let id = u16::from_be_bytes(match &id_bytes[2..4].try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => *value,
    });

    if let Err(error) = source.seek(SeekFrom::Current(4)) {
        return Err(errors::Error::Read(error));
    }

    let mut timed_inners: Option<state::blueprint::time::Instance> = None;
    let mut spatial_inners: Option<state::blueprint::spatial::Instance> = None;

    match type_raw {
        1 => {
            let mut elevation_n_tile_bytes = [0u8; 4];
            match source.read_exact(&mut elevation_n_tile_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            const LEVELS_SCALE: std::ops::Range<u8> = 0u8..3;
            let elevation =
                match u16::from_be_bytes(match &elevation_n_tile_bytes[0..2].try_into() {
                    Err(_) => return Err(errors::Error::Source),
                    Ok(value) => *value,
                }) {
                    0x0000 => Elevation { level: Scaled { value: 0u8, scale: LEVELS_SCALE } },
                    0x2000 => Elevation { level: Scaled { value: 1u8, scale: LEVELS_SCALE } },
                    0x4000 => Elevation { level: Scaled { value: 2u8, scale: LEVELS_SCALE } },
                    _ => return Err(errors::Error::Format(errors::Format::Data))
                };

            let position =
                match Coordinate::try_from(
                    u16::from_be_bytes(match &elevation_n_tile_bytes[2..4].try_into() {
                        Err(_) => return Err(errors::Error::Source),
                        Ok(value) => *value,
                    }) as u32) {
                    Ok(value) => value,
                    Err(error) => return Err(error)
                };

            let mut distance_bytes = [0u8; 4];
            match source.read_exact(&mut distance_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let distance = u32::from_be_bytes(distance_bytes) as u16;

            spatial_inners = Some(state::blueprint::spatial::Instance {
                position,
                distance,
                elevation,
            })
        }
        2 => {
            let mut delay_bytes = [0u8; 4];
            match source.read_exact(&mut delay_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let delay = u32::from_be_bytes(delay_bytes);

            timed_inners = Some(state::blueprint::time::Instance {
                duration: std::time::Duration::new(delay as u64, 0)
            });
        }
        _ => {}
    }

    let mut flags_bytes = [0u8; 4];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let _flags = u32::from_be_bytes(flags_bytes) as u16;

    let mut program_id_bytes = [0u8; 4];
    match source.read_exact(&mut program_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let program_id = u32::from_be_bytes(program_id_bytes);

    if let Err(error) = source.seek(SeekFrom::Current(4)) {
        return Err(errors::Error::Read(error));
    }

    let mut object_id_bytes = [0u8; 4];
    match source.read_exact(&mut object_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let object_id = i32::from_be_bytes(object_id_bytes);

    let mut lvars_offset_bytes = [0u8; 4];
    match source.read_exact(&mut lvars_offset_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let lvars_offset = i32::from_be_bytes(lvars_offset_bytes);

    let mut lvars_count_bytes = [0u8; 4];
    match source.read_exact(&mut lvars_count_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let lvars_count = i32::from_be_bytes(lvars_count_bytes);

    let mut return_value_bytes = [0u8; 4];
    match source.read_exact(&mut return_value_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let _return_value = i32::from_be_bytes(return_value_bytes);

    let mut actions_bytes = [0u8; 4];
    match source.read_exact(&mut actions_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let _actions = i32::from_be_bytes(actions_bytes);

    let mut exit_parameters_bytes = [0u8; 4];
    match source.read_exact(&mut exit_parameters_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let _exit_parameters = i32::from_be_bytes(exit_parameters_bytes);

    let mut actions_count_bytes = [0u8; 4];
    match source.read_exact(&mut actions_count_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let _actions_count = i32::from_be_bytes(actions_count_bytes);

    let mut script_overrides_bytes = [0u8; 4];
    match source.read_exact(&mut script_overrides_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let _script_overrides = i32::from_be_bytes(script_overrides_bytes);

    if let Err(error) = source.seek(SeekFrom::Current(4)) {
        return Err(errors::Error::Read(error));
    }

    let mut how_much_bytes = [0u8; 4];
    match source.read_exact(&mut how_much_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let _how_much = i32::from_be_bytes(how_much_bytes);

    if let Err(error) = source.seek(SeekFrom::Current(4)) {
        return Err(errors::Error::Read(error));
    }

    Ok(state::blueprint::Instance {
        id,
        r#type: match type_raw {
            0 => System,
            1 => Spatial(match spatial_inners {
                Some(value) => value,
                None => return Err(errors::Error::Format(errors::Format::Consistency))
            }),
            2 => Time(match timed_inners {
                Some(value) => value,
                None => return Err(errors::Error::Format(errors::Format::Consistency))
            }),
            3 => Item,
            4 => Critter,
            _ => return Err(errors::Error::Format(errors::Format::Consistency))
        },
        variables: if lvars_offset > -1 && lvars_count > 0 {
            Some(state::blueprint::Variables {
                offset: lvars_offset as u32,
                count: lvars_count as u32,
            })
        } else { None },
        connections: state::blueprint::Connections {
            program_id,
            object_id: if object_id > -1 { Some(object_id as u32) } else { None },
        },
    })
}

pub fn skip<S: Seek>(source: &mut S) -> Result<(), errors::Error> {
    source.seek(SeekFrom::Current(4 * 16)).map(|_| { () }).map_err(|_| { errors::Error::Source })
}