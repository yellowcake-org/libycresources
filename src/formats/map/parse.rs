use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;

use super::*;

mod flags;
mod defaults;
mod variables;
mod tiles;
mod prototypes;
mod scripts;

pub fn map<S: Read + Seek>(source: &mut S) -> Result<Map, errors::Error> {
    source.seek(SeekFrom::Start(0))?;

    let version = source.read_u32::<BigEndian>()?;

    let mut filename_bytes = [0u8; 16];
    source.read_exact(&mut filename_bytes)?;

    let filename = String::from(match std::str::from_utf8(&filename_bytes) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format),
    });

    let defaults = defaults::instance(source)?;
    let local_vars_count = source.read_u32::<BigEndian>()?;
    let _program_id = source.read_i32::<BigEndian>()?;
    let (flags, elevations) = flags::tuple(source)?;
    let darkness = source.read_u32::<BigEndian>()?;
    let global_vars_count = source.read_u32::<BigEndian>()?;

    let id = source.read_u32::<BigEndian>()?;
    let ticks = source.read_u32::<BigEndian>()?;

    source.seek(SeekFrom::Current(4 * 44))?;

    let global_vars = variables::set(source, global_vars_count)?;
    let local_vars = variables::set(source, local_vars_count)?;

    let tiles = tiles::list(source, &elevations)?;
    let scripts = scripts::list(source)?;
    let prototypes = prototypes::list(source, &elevations)?;

    Ok(Map {
        id,
        version,
        filename,
        defaults,
        variables: common::Variables { local: local_vars, global: global_vars },
        flags,
        ticks,
        darkness,
        tiles,
        scripts,
        prototypes,
    })
}