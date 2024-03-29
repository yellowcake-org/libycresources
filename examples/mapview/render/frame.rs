use std::ops::RangeInclusive;

use ycresources::common::types::geometry::Scaled;
use ycresources::formats::frm::Frame;
use ycresources::formats::pal::Palette;

use crate::error::Error;

pub(crate) fn imprint<'a>(
    frame: &Frame,
    palette: &Palette,
    darkness: u8,
    origin: (isize, isize),
    destination: &mut (&mut Vec<(u8, u8, u8)>, (usize, usize)),
) -> Result<(), Error<'a>> {
    let origin = (origin.0 + frame.shift.x as isize, origin.1 + frame.shift.y as isize);

    for (number, &index) in frame.indexes.iter().enumerate() {
        let color = &palette.colors[index as usize];
        let pixel = match color {
            None => None,
            Some(pixel) => {
                fn adjusted(color: &Scaled<u8, RangeInclusive<u8>>, darkness: u8) -> u8 {
                    let max = ((0..u8::MAX).len() / color.scale.len() + 1) as u8;
                    let min = 1;

                    debug_assert!(darkness >= min);
                    debug_assert!(darkness <= max);

                    let effective = std::cmp::min(max, darkness);
                    let effective = std::cmp::max(min, effective);

                    color.value * effective
                }

                let red = adjusted(&pixel.red, darkness);
                let green = adjusted(&pixel.green, darkness);
                let blue = adjusted(&pixel.blue, darkness);

                Some((red, green, blue))
            }
        };

        let (rx, ry) = (
            number as isize % frame.size.width as isize,
            number as isize / frame.size.width as isize
        );

        let (x, y) = origin;
        let (x, y) = (x + rx, y + ry);

        let index = (x + (y * destination.1.0 as isize)) as usize;
        if let Some(pixel) = pixel {
            if destination.0.get(index).is_none() {
                return Err(Error::Corrupted("Object's screen location is out of map bounds"));
            }

            destination.0[index] = pixel;
        }
    }

    Ok(())
}