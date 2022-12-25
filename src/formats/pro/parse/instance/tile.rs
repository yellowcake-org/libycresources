use object::common::world::Material;
use object::tile::Instance;

use super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    Ok(Instance { material: Material::try_from(source.read_u32::<BigEndian>()?)? })
}