use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::scenery::door;

pub(crate) fn patch<S: Read>(source: &mut S, door: &door::Instance) -> Result<door::Patch, errors::Error> {
    todo!()
}