use std::io::{Read, Seek, SeekFrom};

use super::*;
use crate::common::types::geometry::Scaled;

mod r#type;
mod flags;
pub(crate) mod id;
pub mod errors;

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

    let sprite = match object::common::sprite::Reference::try_from(sprite_id_bytes) {
        Ok(value) => value,
        Err(error) => return Err(error)
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

    let light_intensity = u32::from_be_bytes(light_intensity_bytes) as u16;

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