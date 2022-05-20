use std::io::Read;

use super::*;

pub fn set<S: Read>(source: &mut S, count: u32) -> Result<HashSet<i32>, errors::Error> {
    let mut variables = HashSet::new();

    for _ in 0..count {
        variables.insert(source.read_i32::<BigEndian>()?);
    }

    Ok(variables)
}