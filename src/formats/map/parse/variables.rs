use std::io::Read;

use super::*;

pub fn set<S: Read>(source: &mut S, count: u32) -> Result<Vec<i32>, errors::Error> {
    let mut variables = Vec::new();

    for _ in 0..count {
        variables.push(source.read_i32::<BigEndian>()?);
    }

    Ok(variables)
}