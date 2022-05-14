use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::common::types::geometry::Scaled;
use crate::common::types::models::Identifier;

use super::*;

mod r#type;
mod flags;

pub fn prototype<S: Read + Seek>(source: &mut S) -> Result<Prototype, errors::Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(errors::Error::Read(error));
    }

    let identifier = Identifier::try_from(source.read_u32::<BigEndian>()?)?;

    let mut text_id_bytes = [0u8; 4];
    match source.read_exact(&mut text_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let text_id = u32::from_be_bytes(text_id_bytes);

    let sprite = Identifier::try_from(source.read_u32::<BigEndian>()?)?;

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

    let light_intensity = u32::from_be_bytes(light_intensity_bytes) as u16;

    let flags = match flags::instance(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let r#type = match r#type::instance(source, &identifier.kind) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    Ok(Prototype {
        id: identifier.value,
        meta: meta::Info {
            light: meta::info::Light {
                distance: Scaled {
                    value: light_radius,
                    scale: 0..=7,
                },
                intensity: Scaled {
                    value: light_intensity,
                    scale: 0..=u16::MAX,
                },
            },
            flags,
            sprite,
            connections: meta::info::Connections { description_id: text_id },
        },
        r#type,
    })
}