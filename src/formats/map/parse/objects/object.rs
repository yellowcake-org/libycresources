use crate::formats::map::common::{Coordinate, Orientation};

use super::super::*;

pub fn instance<S: Read>(source: &mut S) -> Result<state::object::Instance, errors::Error> {
    let mut id_bytes = [0u8; 4];
    source.read_exact(&mut id_bytes)?;

    let mut position_bytes = [0u8; 4];
    source.read_exact(&mut position_bytes)?;

    let mut sh_x_bytes = [0u8; 4];
    source.read_exact(&mut sh_x_bytes)?;

    let mut sh_y_bytes = [0u8; 4];
    source.read_exact(&mut sh_y_bytes)?;

    let mut sp_x_bytes = [0u8; 4];
    source.read_exact(&mut sp_x_bytes)?;

    let mut sp_y_bytes = [0u8; 4];
    source.read_exact(&mut sp_y_bytes)?;

    let mut frame_idx_bytes = [0u8; 4];
    source.read_exact(&mut frame_idx_bytes)?;

    let mut orientation_bytes = [0u8; 4];
    source.read_exact(&mut orientation_bytes)?;

    let mut sprite_id_bytes = [0u8; 4];
    source.read_exact(&mut sprite_id_bytes)?;

    let mut flags_bytes = [0u8; 4];
    source.read_exact(&mut flags_bytes)?;

    let mut flags_bytes = [0u8; 4];
    source.read_exact(&mut flags_bytes)?;

    let mut elevation_bytes = [0u8; 4];
    source.read_exact(&mut elevation_bytes)?;

    let mut prototype_id_bytes = [0u8; 4];
    source.read_exact(&mut prototype_id_bytes)?;

    let mut prototype_id_bytes = [0u8; 4];
    source.read_exact(&mut prototype_id_bytes)?;

    let mut critter_idx_bytes = [0u8; 4];
    source.read_exact(&mut critter_idx_bytes)?;

    let mut light_radius_bytes = [0u8; 4];
    source.read_exact(&mut light_radius_bytes)?;

    let mut light_intensity_bytes = [0u8; 4];
    source.read_exact(&mut light_intensity_bytes)?;

    let mut outline_color_bytes = [0u8; 4];
    source.read_exact(&mut outline_color_bytes)?;

    let mut script_id_bytes = [0u8; 4];
    source.read_exact(&mut script_id_bytes)?;

    let mut inventory_count_bytes = [0u8; 4];
    source.read_exact(&mut inventory_count_bytes)?;

    let mut inventory_capacity_bytes = [0u8; 4];
    source.read_exact(&mut inventory_capacity_bytes)?;

    let id = u32::from_be_bytes(id_bytes);
    let position = Coordinate::try_from(u32::from_be_bytes(position_bytes))?;

    let sh_x = i32::from_be_bytes(sh_x_bytes);
    let sh_y = i32::from_be_bytes(sh_y_bytes);

    let sp_x = i32::from_be_bytes(sp_x_bytes);
    let sp_y = i32::from_be_bytes(sp_y_bytes);

    let sprite_id = u32::from_be_bytes(elevation_bytes);
    let frame_idx = u32::from_be_bytes(frame_idx_bytes);
    let orientation = Orientation::try_from(u32::from_be_bytes(orientation_bytes))?;

    Ok(state::object::Instance { reference_id: u32::from_be_bytes(prototype_id_bytes) })
}