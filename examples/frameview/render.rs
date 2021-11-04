use libycresources::formats::{frm, pal};

pub fn frame(frame: &frm::Frame, palette: &pal::Palette) -> bmp::Image {
    let mut image = bmp::Image::new(frame.size.width as u32, frame.size.height as u32);

    for (x, y) in image.coordinates() {
        let index = frame.pixels[(frame.size.width as u32 * y + x) as usize];
        let pixel = &palette.colors[index as usize];

        image.set_pixel(
            x,
            y,
            match pixel {
                None => bmp::Pixel::new(0, 0, 0),
                Some(color) => {
                    let red = ((color.red.value * (std::u8::MAX as usize + 1))
                        / ((color.red.scale.end - color.red.scale.start) as usize))
                        as u8;
                    let green = ((color.green.value * (std::u8::MAX as usize + 1))
                        / ((color.green.scale.end - color.green.scale.start) as usize))
                        as u8;
                    let blue = ((color.blue.value * (std::u8::MAX as usize + 1))
                        / ((color.blue.scale.end - color.blue.scale.start) as usize))
                        as u8;

                    bmp::Pixel::new(red, green, blue)
                }
            },
        );
    }

    image
}
