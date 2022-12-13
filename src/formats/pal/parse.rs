use std::io::{Read, Seek, SeekFrom};

use byteorder::ReadBytesExt;

use crate::common::graphics::Pixel;
use crate::common::types::errors::Error;
use crate::common::types::geometry::Scaled;

use super::Palette;

pub fn palette<S: Read + Seek>(source: &mut S) -> Result<Palette, Error> {
    source.seek(SeekFrom::Start(0))?;

    let scale = u8::MIN..=63;
    let mut colors = [(u8::MIN, u8::MIN, u8::MIN, false); 256];

    for color in &mut colors {
        let red = source.read_u8()?;
        let green = source.read_u8()?;
        let blue = source.read_u8()?;

        if scale.contains(&red) &&
            scale.contains(&green) &&
            scale.contains(&blue) {
            *color = (red, green, blue, true);
        }
    }

    let colors = colors.map(|(red, green, blue, is_regular)| {
        if is_regular {
            Some(Pixel {
                red: Scaled {
                    value: red,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: green,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: blue,
                    scale: scale.clone(),
                },
            })
        } else {
            None
        }
    });

    Ok(Palette { colors })
}
