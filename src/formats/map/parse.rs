use std::io::{Read, Seek, SeekFrom};

use crate::common::types::ScaledValue;

use super::*;

pub mod errors;

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

    let mut default_position_bytes = [0u8; 4];
    match source.read_exact(&mut default_position_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let default_position = match u32::from_be_bytes(default_position_bytes) {
        value if (0..40000).contains(&value) => ScaledValue { value: value as u16, scale: 0u16..40000 },
        _ => return Err(errors::Error::Format(errors::Format::Data))
    };

    let mut default_elevation_bytes = [0u8; 4];
    match source.read_exact(&mut default_elevation_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let default_elevation = match u32::from_be_bytes(default_elevation_bytes) {
        value if (0..3).contains(&value) => ScaledValue { value: value as u8, scale: 0u8..3 },
        _ => return Err(errors::Error::Format(errors::Format::Data))
    };

    let mut default_orientation_bytes = [0u8; 4];
    match source.read_exact(&mut default_orientation_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let default_orientation = match u32::from_be_bytes(default_orientation_bytes) {
        value if (0..6).contains(&value) => ScaledValue { value: value as u8, scale: 0u8..6 },
        _ => return Err(errors::Error::Format(errors::Format::Data))
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

    let mut flags_bytes = [0u8; 4];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };


    let mut flags: HashSet<common::Flag> = HashSet::new();
    let mut elevations: HashSet<u8> = HashSet::new();

    if (flags_bytes[3] & 0x01) != 0x00 {
        if !flags.insert(common::Flag::Save) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[3] & 0x02) == 0x00 {
        if !elevations.insert(0) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[3] & 0x04) == 0x00 {
        if !elevations.insert(1) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[3] & 0x08) == 0x00 {
        if !elevations.insert(2) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

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

    Ok(Map {
        id,
        version,
        filename,
        defaults: common::Defaults {
            position: default_position,
            elevation: default_elevation,
            orientation: default_orientation
        },
        flags,
        elevations,
        ticks,
        darkness
    })
}