use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::geometry::{Coordinate, Elevation, Orientation, Scaled};
use crate::common::types::models;
use crate::formats::map::blueprint;
use crate::formats::map::parse::{errors, PrototypeProvider};
use crate::formats::pro::meta;
use crate::formats::pro::meta::info::Light;

mod patch;

pub fn instance<S: Read + Seek, P: PrototypeProvider>(source: &mut S, provider: &P) ->
Result<blueprint::prototype::Instance, errors::Error> {
    let _entry_id = source.read_u32::<BigEndian>()?;

    let position = u32::try_from(source.read_i32::<BigEndian>()?)
        .ok()
        .map_or(Ok(None), |v| {
            Coordinate::try_from(v).map(|c| Some(c))
        })?;

    let screen_shift = Coordinate {
        x: Scaled { value: source.read_u32::<BigEndian>()?, scale: u32::MIN..u32::MAX },
        y: Scaled { value: source.read_u32::<BigEndian>()?, scale: u32::MIN..u32::MAX },
    };

    let screen_position = Coordinate {
        x: Scaled { value: source.read_i32::<BigEndian>()?, scale: i32::MIN..i32::MAX },
        y: Scaled { value: source.read_i32::<BigEndian>()?, scale: i32::MIN..i32::MAX },
    };

    let frame_idx = source.read_u32::<BigEndian>()?;
    let orientation = Orientation::try_from(source.read_u32::<BigEndian>()?)?;

    let sprite_id = source.read_u32::<BigEndian>()?;

    let flags = source.read_u32::<BigEndian>()?;
    let elevation = Elevation::try_from(source.read_u32::<BigEndian>()?)?;

    let identifier = models::Identifier::try_from(source.read_u32::<BigEndian>()?)?;
    let critter_idx = source.read_i32::<BigEndian>()?;

    let light_radius = source.read_u32::<BigEndian>()?;
    let light_intensity = source.read_u32::<BigEndian>()?;

    let outline_color = source.read_u32::<BigEndian>()?;

    let script_id = source.read_i32::<BigEndian>()?;
    let program_id = source.read_i32::<BigEndian>()?;

    let inventory_items_count = source.read_u32::<BigEndian>()?;
    let inventory_items_capacity = source.read_u32::<BigEndian>()?;

    source.seek(SeekFrom::Current(4))?;

    let flags_patch = source.read_u32::<BigEndian>()?;
    let patch = patch::instance(source, provider, &identifier)?;

    let mut inventory = Vec::new();

    for _ in u32::MIN..inventory_items_capacity { inventory.push(None) }
    for _ in u32::MIN..inventory_items_count {
        let index = usize::try_from(source.read_u32::<BigEndian>()?).map_err(|_| errors::Error::Format)?;

        // Sometimes we face here an inventory item which have index
        // greater than original capacity of the inventory itself, so we grow the vec in this case
        let overhead = i32::max(index as i32 - (inventory_items_capacity - 1) as i32, 0) as usize;
        for _ in usize::MIN..overhead { inventory.push(None) }

        // Now this operation is safe from panic
        inventory[index as usize] = Some(self::instance(source, provider)?);
    }

    Ok(blueprint::prototype::Instance {
        identifier,
        patch: blueprint::prototype::Patch {
            meta: meta::Patch {
                light: Light::try_from((light_radius as u8, light_intensity as u16))?,
                flags: Default::default(),
            },
            object: patch,
        },
        inventory,
    })
}