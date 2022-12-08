use bmp::Image;

use libycresources::formats::frm::Frame;
use libycresources::formats::pal::Palette;

pub(crate) fn imprint(frame: &Frame, palette: &Palette, origin: (usize, usize), destination: &mut Image) {
    let origin = (origin.0 + frame.shift.x as usize, origin.1 + frame.shift.y as usize);

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
            number % frame.size.width as usize,
            number / frame.size.width as usize
        );

        let (x, y) = origin;
        let (x, y) = (x + rx, y + ry);

        if let Some(pixel) = pixel {
            destination.set_pixel(x as u32, y as u32, pixel);
        }
    }
}