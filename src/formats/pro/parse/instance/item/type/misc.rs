use object::common::weapons::Caliber;
use object::item::misc::{Connections, Instance};

use crate::common::traits::TryFromOptional;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let item_pid = u32::try_from_optional(source.read_i32::<BigEndian>()?, -1)
        .map_err(|_| errors::Error::Format)?;

    let caliber = Caliber::try_from_optional(source.read_u32::<BigEndian>()?, 0)?;
    let count = source.read_u32::<BigEndian>()?;

    Ok(Instance {
        count,
        caliber,
        connections: Connections { power_item_id: item_pid },
    })
}