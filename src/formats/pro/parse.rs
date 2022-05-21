use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::common::types::models::Identifier;
use crate::formats::pro::meta::info::Light;

use super::*;

mod instance;
pub(crate) mod flags;

pub fn prototype<S: Read + Seek>(source: &mut S) -> Result<Prototype, errors::Error> {
    source.seek(SeekFrom::Start(0))?;

    let identifier = Identifier::try_from(source.read_u32::<BigEndian>()?)?;
    let description_id = source.read_u32::<BigEndian>()?;
    let sprite = Identifier::try_from(source.read_u32::<BigEndian>()?)?;

    let light_radius = source.read_u32::<BigEndian>()? as u8;
    let light_intensity = source.read_u32::<BigEndian>()? as u16;

    let flags = flags::common(source)?;
    let object = instance::instance(source, &identifier.kind)?;

    Ok(Prototype {
        id: identifier.value,
        meta: meta::Instance {
            light: Light::try_from((light_radius, light_intensity))?,
            flags,
            sprite,
            connections: meta::info::Connections { description_id },
        },
        object,
    })
}