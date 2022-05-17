use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::scenery;

pub(crate) fn patch<S: Read>(source: &mut S, scenery: &scenery::Instance) -> Result<scenery::Patch, errors::Error> {
    todo!()
}