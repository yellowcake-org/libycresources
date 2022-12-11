use bmp::Image;

use libycresources::formats::frm::Frame;
use libycresources::formats::pal::Palette;

pub(crate) fn imprint(frame: &Frame, palette: &Palette, origin: (isize, isize), destination: &mut Image) {
    let origin = (origin.0 + frame.shift.x as isize, origin.1 + frame.shift.y as isize);

    for (number, &index) in frame.indexes.iter().enumerate() {
        let color = &palette.colors[index as usize];
        let pixel = match color {
            None => None,
            Some(c) => {
                let red = ((c.red.value as usize * (u8::MAX as usize + 1)) / c.red.scale.len()) as u8;
                let green = ((c.green.value as usize * (u8::MAX as usize + 1)) / c.green.scale.len()) as u8;
                let blue = ((c.blue.value as usize * (u8::MAX as usize + 1)) / c.blue.scale.len()) as u8;

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