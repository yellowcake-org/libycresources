use std::collections::HashSet;

use super::super::super::*;

pub(crate) fn extract(from: u8) -> Result<HashSet<object::common::actions::Instance>, errors::Error> {
    let mut actions: HashSet<object::common::actions::Instance> = HashSet::new();

    if (from & 0x80) == 0x80 &&
        !actions.insert(object::common::actions::Instance::PickUp) {
        return Err(errors::Error::Format);
    }

    let can_use = (from & 0x08) == 0x08;
    let can_use_on = (from & 0x10) == 0x10;

    let usage = object::common::actions::Usage {
        itself: can_use,
        something: can_use_on,
        knees_down: false,
    };

    if can_use || can_use_on {
        if !actions.insert(object::common::actions::Instance::Usage(usage)) {
            return Err(errors::Error::Format);
        }
    }

    Ok(actions)
}