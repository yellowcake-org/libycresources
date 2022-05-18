use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::scenery::stairs;

pub(crate) fn patch<S: Read>(source: &mut S, stairs: &stairs::Instance) -> Result<stairs::Patch, errors::Error> {
    todo!()
}