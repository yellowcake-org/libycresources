use std::io::Read;

use super::*;

pub fn list<S: Read>(source: &mut S) -> Result<HashSet<state::blueprints::Instance>, errors::Error> {
    let mut list = HashSet::new();

    Ok(list)
}