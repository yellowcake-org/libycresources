use super::*;

mod object;

pub fn list<S: Read>(source: &mut S) -> Result<HashSet<state::object::Instance>, errors::Error> {
    let mut list = HashSet::new();
    Ok(list)
}