use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::scenery::ladder;

pub(crate) fn patch<S: Read>(source: &mut S, ladder: &ladder::Instance) -> Result<ladder::Patch, errors::Error> {
    todo!()
}