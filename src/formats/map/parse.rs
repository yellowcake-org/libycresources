use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::common::types::models::Identifier;
use crate::formats::pro;

use super::*;

mod flags;
mod entrance;
mod variables;
mod tiles;
mod prototypes;
mod scripts;

pub trait PrototypeProvider {
    fn provide(&self, identifier: &Identifier<pro::ObjectType>) -> Result<pro::Prototype, errors::Error>;
}

pub fn map<S: Read + Seek, P: PrototypeProvider>(source: &mut S, provider: &P) -> Result<Map, errors::Error> {
    source.seek(SeekFrom::Start(0))?;

    let version = source.read_u32::<BigEndian>()?;
    let read_ladders_map = version == 20; // Fallout 2 maps

    let mut filename_bytes = [0u8; 16];
    source.read_exact(&mut filename_bytes)?;

    let filename = String::from(
        std::str::from_utf8(&filename_bytes).map_err(|_| errors::Error::Format)?
    );

    let entrance = entrance::instance(source)?;
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
    let prototypes = prototypes::list(source, provider, &elevations, read_ladders_map)?;

    Ok(Map {
        id,
        version,
        filename,
        entrance,
        variables: common::Variables { local: local_vars, global: global_vars },
        flags,
        ticks,
        darkness,
        tiles,
        scripts,
        prototypes,
    })
}