use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::scenery::elevator;

pub(crate) fn patch<S: Read>(source: &mut S, elevator: &elevator::Instance) -> Result<elevator::Patch, errors::Error> {
    todo!()
}