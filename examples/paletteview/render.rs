use libycresources::common::graphics::ColorPixel;

pub fn image(pixels: &[Option<ColorPixel>], width: usize) -> bmp::Image {
    let height = pixels.len() / width;

    let pixels: Vec<bmp::Pixel> = pixels
        .iter()
        .map(|color| match color {
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
        })
        .collect();

    let mut image = bmp::Image::new(width as u32, height as u32);

    for (x, y) in image.coordinates() {
        image.set_pixel(x, y, pixels[(width as u32 * y + x) as usize]);
    }

    image
}
