use std::io::{Read, Seek, SeekFrom};
use std::ops::RangeInclusive;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::geometry::{Coordinate, Orientation, Scaled};
use crate::common::types::models;
use crate::common::types::models::Identifier;
use crate::common::types::space::Elevation;
use crate::formats::map::blueprint::prototype;
use crate::formats::map::blueprint::prototype::Appearance;
use crate::formats::map::location::{Grid, Screen};
use crate::formats::map::parse::{errors, Provider};
use crate::formats::pro;
use crate::formats::pro::meta;
use crate::formats::pro::meta::info::Light;

mod patch;

pub fn instance<S: Read + Seek, P: Provider>(source: &mut S, provider: &P, read_ladders_map: bool) ->
Result<prototype::Instance, errors::Error> {
    let _entry_id = source.read_u32::<BigEndian>()?;

    let position = u32::try_from(source.read_i32::<BigEndian>()?)
        .ok()
        .map_or(Ok(None), |v| {
            Coordinate::try_from(v).map(|c| Some(c))
        })?;

    fn screen<S: Read>(source: &mut S) -> Result<Coordinate<i32, RangeInclusive<i32>>, errors::Error> {
        Ok(Coordinate {
            x: Scaled { value: source.read_i32::<BigEndian>()?, scale: i32::MIN..=i32::MAX },
            y: Scaled { value: source.read_i32::<BigEndian>()?, scale: i32::MIN..=i32::MAX },
        })
    }

    let screen_shift = screen(source)?;
    let screen_position = screen(source)?;

    let frame_idx = source.read_u32::<BigEndian>()?;
    let orientation = Orientation::try_from(source.read_u32::<BigEndian>()?)?;

    let sprite = Identifier::try_from(source.read_u32::<BigEndian>()?)?;

    let mut flags = pro::parse::flags::common(source)?;
    let elevation = Elevation::try_from(source.read_u32::<BigEndian>()?)?;

    let location = prototype::Location {
        grid: position.map(|v| Grid { position: v, elevation, orientation }),
        screen: Screen { position: screen_position, correction: screen_shift },
    };

    let identifier = models::Identifier::try_from(source.read_u32::<BigEndian>()?)?;
    let _critter_idx = source.read_i32::<BigEndian>()?;

    let light_radius = source.read_u32::<BigEndian>()?;
    let light_intensity = source.read_u32::<BigEndian>()?;

    let _outline_color = source.read_u32::<BigEndian>()?;

    let _script_id = source.read_i32::<BigEndian>()?;
    let _program_id = source.read_i32::<BigEndian>()?;

    let inventory_items_count = source.read_u32::<BigEndian>()?;
    let inventory_items_capacity = source.read_u32::<BigEndian>()?;

    source.seek(SeekFrom::Current(4))?;

    flags.extend(pro::parse::flags::extended(source)?);
    let patch = patch::instance(source, provider, &identifier, read_ladders_map)?;

    let mut inventory = Vec::new();
    for _ in u32::MIN..inventory_items_capacity { inventory.push(None) }

    // Fill specific slots with objects
    for _ in u32::MIN..inventory_items_count {
        let index = usize::try_from(source.read_u32::<BigEndian>()?).map_err(|_| errors::Error::Format)?;

        // Sometimes we face here an inventory item which have index
        // greater than original capacity of the inventory itself, so we grow the vec in this case
        let overhead = i32::max(index as i32 - (inventory_items_capacity - 1) as i32, 0) as usize;
        for _ in usize::MIN..overhead { inventory.push(None) }

        // Now this operation is safe from panic
        inventory[index] = Some(instance(source, provider, read_ladders_map)?);
    }

    Ok(prototype::Instance {
        id: identifier,
        patch: prototype::Patch {
            meta: meta::Patch {
                light: Light::try_from((light_radius as u8, light_intensity as u16))?,
                flags,
            },
            object: patch,
        },
        location,
        appearance: Appearance {
            current: if frame_idx > 0 {
                Some(u16::try_from(frame_idx).map_err(|_| errors::Error::Format)?)
            } else { None },
            sprite,
        },
        inventory,
    })
}