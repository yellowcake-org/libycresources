use super::super::*;

pub fn instance<S: Read>(source: &mut S, type_raw: u32) -> Result<state::blueprints::Instance, errors::Error> {
    todo!()
}

pub fn skip<S: Seek>(source: &mut S, type_raw: u32) -> Result<(), errors::Error> {
    source.seek(SeekFrom::Current(4 * (16 + match type_raw {
        1 => 2,
        2 => 1,
        _ => 0
    }))).map(|_| { () }).map_err(|_| { errors::Error::Source })
}