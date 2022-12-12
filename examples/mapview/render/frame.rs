use std::ops::RangeInclusive;

use bmp::Image;

use libycresources::common::types::geometry::Scaled;
use libycresources::formats::frm::Frame;
use libycresources::formats::pal::Palette;

pub(crate) fn imprint(frame: &Frame, palette: &Palette, darkness: u8, origin: (isize, isize), destination: &mut Image) {
    let origin = (origin.0 + frame.shift.x as isize, origin.1 + frame.shift.y as isize);

    for (number, &index) in frame.indexes.iter().enumerate() {
        let color = &palette.colors[index as usize];
        let pixel = match color {
            None => None,
            Some(pixel) => {
                fn adjusted(color: &Scaled<u8, RangeInclusive<u8>>, darkness: u8) -> u8 {
                    let max = ((0..u8::MAX).len() / color.scale.len()) as u8;
                    let min = 1;

                    let effective = std::cmp::min(max, darkness);
                    let effective = std::cmp::max(min, effective);

                    color.value * if effective == 1 { max } else { effective }
                }

                let red = adjusted(&pixel.red, darkness);
                let green = adjusted(&pixel.green, darkness);
                let blue = adjusted(&pixel.blue, darkness);

                Some(bmp::Pixel::new(red, green, blue))
            }
        };

        let (rx, ry) = (
            number as isize % frame.size.width as isize,
            number as isize / frame.size.width as isize
        );

        let (x, y) = origin;
        let (x, y) = (x + rx, y + ry);

        if let Some(pixel) = pixel { destination.set_pixel(x as u32, y as u32, pixel); }
    }
}