use std::collections::HashSet;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::formats::pro::object::scenery::door::Flag::Passable;
use crate::formats::pro::object::scenery::door::Patch;

pub(crate) fn patch<S: Read>(source: &mut S) -> Result<Patch, errors::Error> {
    let flags = source.read_u32::<BigEndian>()?;
    let mut set = HashSet::new();

    if (flags >> u32::BITS * 0) & 0x0F == 0x0F {
        set.insert(Passable);
    }

    Ok(Patch { flags: set })
}