use object::item::key::Instance;
use crate::common::traits::TryFromOptional;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let code = u32::try_from_optional(source.read_i32::<BigEndian>()?, -1)
        .map_err(|_| errors::Error::Format)?;

    Ok(Instance { code })
}