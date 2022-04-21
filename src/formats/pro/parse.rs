use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;
use std::time::Duration;

use super::*;
use super::super::super::common::types::ScaledValue;

mod r#type;
mod flags;
pub(crate) mod id;

pub mod errors {
    #[derive(Debug)]
    pub enum Format {
        Type,
        Data,
        Flags,
        Consistency,
    }

    #[derive(Debug)]
    pub enum Error {
        Read(std::io::Error),
        Format(Format),
        Source,
    }
}

pub fn prototype<S: Read + Seek>(source: &mut S) -> Result<Prototype, errors::Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(errors::Error::Read(error));
    }

    let mut id_bytes = [0u8; 4];
    match source.read_exact(&mut id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let (type_id, object_id) = match id::instance(id_bytes) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let mut text_id_bytes = [0u8; 4];
    match source.read_exact(&mut text_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let text_id = u32::from_be_bytes(text_id_bytes);

    let mut sprite_id_bytes = [0u8; 4];
    match source.read_exact(&mut sprite_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut light_radius_bytes = [0u8; 4];
    match source.read_exact(&mut light_radius_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let light_radius = light_radius_bytes[3];

    let mut light_intensity_bytes = [0u8; 4];
    match source.read_exact(&mut light_intensity_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let light_intensity =
        u16::from_be_bytes(match &light_intensity_bytes[2..4].try_into() {
            Ok(value) => *value,
            Err(_) => return Err(errors::Error::Source),
        });

    let flags = match flags::instance(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let r#type = match r#type::instance(source, type_id) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    Ok(Prototype {
        id: object_id,
        meta: meta::Info {
            light: meta::info::Light {
                distance: ScaledValue {
                    value: light_radius,
                    scale: Range { start: 0, end: 8 },
                },
                intensity: ScaledValue {
                    value: light_intensity,
                    scale: Range { start: 0, end: u16::MAX },
                },
            },
            flags,
            sprite: match object::common::sprite::Reference::try_from(sprite_id_bytes) {
                Ok(value) => value,
                Err(error) => return Err(error)
            },
            connections: meta::info::Connections { description_id: text_id },
        },
        r#type,
    })
}