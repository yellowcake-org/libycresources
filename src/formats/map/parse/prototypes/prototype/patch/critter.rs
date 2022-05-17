use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::critter;

pub(crate) fn patch<S: Read>(source: &mut S) -> Result<critter::Patch, errors::Error> {
    todo!()
}