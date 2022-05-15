use std::io::Read;

use super::*;
use super::super::tiles::Instance;

pub fn list<S: Read>(source: &mut S, elevations: &[Option<()>; 3]) -> Result<[Option<Instance>; 3], errors::Error> {
    let mut result = [None, None, None];

    for (idx, e) in elevations.iter().enumerate() {
        result[idx] = match e {
            None => None,
            Some(_) => {
                let mut tiles = Instance {
                    roof: [[None; 100]; 100],
                    floor: [[None; 100]; 100],
                };

                for horizontal in 0..tiles.roof.len() {
                    for vertical in 0..tiles.roof[0].len() {
                        fn tile_id<S: Read>(source: &mut S) -> Result<Option<u16>, errors::Error> {
                            return Ok(match source.read_u16::<BigEndian>()? {
                                1 => None,
                                v => Some(v)
                            });
                        }

                        tiles.roof[horizontal][vertical] = tile_id(source)?;
                        tiles.floor[horizontal][vertical] = tile_id(source)?;
                    }
                }

                Some(tiles)
            }
        }
    }

    Ok(result)
}