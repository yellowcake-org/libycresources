use std::io::Read;

use crate::common::types::errors;
use crate::formats::pro::object::item;

pub(crate) mod misc;

pub(crate) fn patch<S: Read>(source: &mut S, item: &item::Instance) -> Result<item::ItemPatch, errors::Error> {
    todo!()
}