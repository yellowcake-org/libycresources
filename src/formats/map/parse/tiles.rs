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

                for horizontal in 0..100 {
                    for vertical in 0..100 {
                        let mut roof_id_bytes = [0u8; 2];
                        match source.read_exact(&mut roof_id_bytes) {
                            Err(error) => return Err(errors::Error::Read(error)),
                            Ok(value) => value,
                        };

                        tiles.roof[horizontal][vertical] = match u16::from_be_bytes(roof_id_bytes) {
                            1 => None,
                            v => Some(v)
                        };

                        let mut floor_id_bytes = [0u8; 2];
                        match source.read_exact(&mut floor_id_bytes) {
                            Err(error) => return Err(errors::Error::Read(error)),
                            Ok(value) => value,
                        };

                        tiles.floor[horizontal][vertical] = match u16::from_be_bytes(floor_id_bytes) {
                            1 => None,
                            v => Some(v)
                        };
                    }
                }

                Some(tiles)
            }
        }
    }

    Ok(result)
}