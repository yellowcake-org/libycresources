use std::collections::HashSet;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::formats::pro::object::scenery::door::Patch;

pub(crate) fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    // TODO: Rework flags parsing!
    let flags = source.read_u32::<BigEndian>()?;
    Ok(Patch { flags: HashSet::new() })
}