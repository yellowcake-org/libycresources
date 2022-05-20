use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors;
use crate::common::types::geometry::{Coordinate, Elevation, Scaled};
use crate::formats::map::tiles;

pub fn list<S: Read>(source: &mut S, elevations: &[Option<()>; 3]) -> Result<Vec<tiles::Group>, errors::Error> {
    let mut result = Vec::new();

    for (idx, e) in elevations.iter().enumerate() {
        match e {
            None => {}
            Some(_) => {
                const SIDE_LEN: u8 = 100;

                let mut floor = Vec::new();
                let mut ceiling = Vec::new();

                let elevation = Elevation {
                    level: Scaled {
                        value: u8::try_from(idx).map_err(|_| errors::Error::Format)?,
                        scale: u8::MIN..u8::try_from(elevations.len()).map_err(|_| errors::Error::Format)?,
                    }
                };

                for horizontal in 0..SIDE_LEN {
                    for vertical in 0..SIDE_LEN {
                        fn consume<S: Read>(source: &mut S, into: &mut Vec<tiles::Instance>, x: u8, y: u8) ->
                        Result<(), errors::Error> {
                            let id = source.read_u16::<BigEndian>()?;

                            if id > 1 {
                                into.push(tiles::Instance {
                                    id,
                                    position: Coordinate {
                                        x: Scaled { value: x, scale: u8::MIN..SIDE_LEN },
                                        y: Scaled { value: y, scale: u8::MIN..SIDE_LEN },
                                    },
                                })
                            }

                            Ok(())
                        }

                        consume(source, &mut ceiling, horizontal, vertical)?;
                        consume(source, &mut floor, horizontal, vertical)?;
                    }
                }

                result.push(tiles::Group { floor, ceiling, elevation });
            }
        }
    }

    Ok(result)
}