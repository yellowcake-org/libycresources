use ycresources::common::graphics::Pixel;

pub fn image(pixels: &[Option<Pixel<u8>>], width: usize) -> bmp::Image {
    let height = pixels.len() / width;

    let pixels: Vec<bmp::Pixel> = pixels
        .iter()
        .map(|color| match color {
            None => bmp::Pixel::new(0, 0, 0),
            Some(color) => {
                let red = ((color.red.value as usize * (u8::MAX as usize + 1)) / color.red.scale.len()) as u8;
                let green = ((color.green.value as usize * (u8::MAX as usize + 1)) / color.green.scale.len()) as u8;
                let blue = ((color.blue.value as usize * (u8::MAX as usize + 1)) / color.blue.scale.len()) as u8;

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
