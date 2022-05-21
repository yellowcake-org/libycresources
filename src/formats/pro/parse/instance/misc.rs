use object::misc::Instance;
use super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    Ok(Instance { _unknown: source.read_u32::<BigEndian>()? })
}