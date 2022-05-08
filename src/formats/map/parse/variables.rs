use std::io::Read;

use super::*;

pub fn set<S: Read>(source: &mut S, count: u32) -> Result<HashSet<i32>, errors::Error> {
    let mut variables = HashSet::new();

    for _ in 0..count {
        let mut variable_bytes = [0u8; 4];
        match source.read_exact(&mut variable_bytes) {
            Err(error) => return Err(errors::Error::Read(error)),
            Ok(value) => value,
        };

        variables.insert(i32::from_be_bytes(variable_bytes));
    }

    Ok(variables)
}