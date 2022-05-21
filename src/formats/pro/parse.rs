use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::common::types::geometry::Scaled;
use crate::common::types::models::Identifier;
use crate::formats::pro::meta::info::Light;

use super::*;

mod r#type;
pub(crate) mod flags;

pub fn prototype<S: Read + Seek>(source: &mut S) -> Result<Prototype, errors::Error> {
    source.seek(SeekFrom::Start(0))?;

    let identifier = Identifier::try_from(source.read_u32::<BigEndian>()?)?;
    let text_id = source.read_u32::<BigEndian>()?;
    let sprite = Identifier::try_from(source.read_u32::<BigEndian>()?)?;

    let mut light_radius_bytes = [0u8; 4];
    match source.read_exact(&mut light_radius_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    let light_radius = light_radius_bytes[3];

    let mut light_intensity_bytes = [0u8; 4];
    match source.read_exact(&mut light_intensity_bytes) {
        Err(error) => return Err(errors::Error::IO(error)),
        Ok(value) => value,
    };

    let light_intensity = u32::from_be_bytes(light_intensity_bytes) as u16;

    let flags = match flags::common(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let r#type = match r#type::instance(source, &identifier.kind) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    Ok(Prototype {
        id: identifier.value,
        meta: meta::Instance {
            light: Light::try_from((light_radius as u8, light_intensity as u16))?,
            flags,
            sprite,
            connections: meta::info::Connections { description_id: text_id },
        },
        object: r#type,
    })
}