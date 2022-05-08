use std::io::{Read, Seek, SeekFrom};

use super::*;

mod errors;

mod flags;
mod defaults;
mod variables;
mod tiles;

pub fn map<S: Read + Seek>(source: &mut S) -> Result<Map, errors::Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(errors::Error::Read(error));
    }

    let mut version_bytes = [0u8; 4];
    match source.read_exact(&mut version_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let version = u32::from_be_bytes(version_bytes);

    let mut filename_bytes = [0u8; 16];
    match source.read_exact(&mut filename_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let filename = String::from(match std::str::from_utf8(&filename_bytes) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
    });

    let defaults = match defaults::instance(source) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let mut local_vars_count_bytes = [0u8; 4];
    match source.read_exact(&mut local_vars_count_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let local_vars_count = u32::from_be_bytes(local_vars_count_bytes);

    let mut script_id_bytes = [0u8; 4];
    match source.read_exact(&mut script_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let _script_id = i32::from_be_bytes(script_id_bytes);

    let (flags, elevations) = match flags::tuple(source) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let mut darkness_bytes = [0u8; 4];
    match source.read_exact(&mut darkness_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let darkness = u32::from_be_bytes(darkness_bytes);

    let mut global_vars_count_bytes = [0u8; 4];
    match source.read_exact(&mut global_vars_count_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let global_vars_count = u32::from_be_bytes(global_vars_count_bytes);

    let mut id_bytes = [0u8; 4];
    match source.read_exact(&mut id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let id = u32::from_be_bytes(id_bytes);

    let mut ticks_bytes = [0u8; 4];
    match source.read_exact(&mut ticks_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ticks = u32::from_be_bytes(ticks_bytes);

    if let Err(error) = source.seek(SeekFrom::Current(4 * 44)) {
        return Err(errors::Error::Read(error));
    }

    let global_vars = match variables::set(source, global_vars_count) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let local_vars = match variables::set(source, local_vars_count) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let tiles = match tiles::tiles(source, &elevations) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

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
    })
}