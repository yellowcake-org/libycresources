use libycresources::formats::{frm, pal};

#[derive(Debug)]
pub enum Error { Corrupted }

pub fn frame(frame: &frm::Frame, palette: &pal::Palette) -> Result<bmp::Image, Error> {
    let mut image = bmp::Image::new(frame.size.width as u32, frame.size.height as u32);

    for (x, y) in image.coordinates() {
        let index = (frame.size.width as u32 * y + x) as usize;

        if let Some(&index) = frame.indexes.get(index) {
            if let Some(&index) = frame.indexes.get(index as usize) {
                if let Some(pixel) = &palette.colors.get(index as usize) {
                    image.set_pixel(
                        x,
                        y,
                        match pixel {
                            None => bmp::Pixel::new(0, 0, 0),
                            Some(color) => {
                                let red = ((color.red.value as usize * (u8::MAX as usize + 1)) / color.red.scale.len()) as u8;
                                let green = ((color.green.value as usize * (u8::MAX as usize + 1)) / color.green.scale.len()) as u8;
                                let blue = ((color.blue.value as usize * (u8::MAX as usize + 1)) / color.blue.scale.len()) as u8;

                                bmp::Pixel::new(red, green, blue)
                            }
                        },
                    );
                } else {
                    return Err(Error::Corrupted);
                }
            } else {
                return Err(Error::Corrupted);
            }
        } else {
            return Err(Error::Corrupted);
        }
    }

    Ok(image)
}
