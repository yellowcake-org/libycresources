use super::super::*;
use super::super::super::traits::TryFromOptional;

mod flags;
mod parameters;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::critter::Instance, errors::Error> {
    let mut flags_bytes = [0u8; 4];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut script_id_bytes = [0u8; 4];
    match source.read_exact(&mut script_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let script =
        match object::common::script::Reference::
        try_from_optional(script_id_bytes, [0xFF, 0xFF, 0xFF, 0xFF]) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        };

    let mut sprite_id_bytes = [0u8; 4];
    match source.read_exact(&mut sprite_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let sprite = match object::common::sprite::Reference::
    try_from_optional(sprite_id_bytes, [0xFF, 0xFF, 0xFF, 0xFF]) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
    };

    let mut ai_packet_id_bytes = [0u8; 4];
    match source.read_exact(&mut ai_packet_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ai_packet_id = u32::from_be_bytes(ai_packet_id_bytes);

    let mut team_num_bytes = [0u8; 4];
    match source.read_exact(&mut team_num_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let team_num = u32::from_be_bytes(team_num_bytes);

    let flags = flags::instance(source);
    let basic = parameters::instance(source);
    let bonuses = parameters::instance(source);

    Err(errors::Error::Source)
}