use std::cmp::min;

use bmp::{Image, Pixel};

use libycresources::common::types::errors::Error;

pub(crate) fn overlay(
    image: &mut Image,
) -> Result<(), Error> {
    let (tw, th, sh) = (32usize, 16usize, 8usize);

    let (gw, gh) = (image.get_width() as usize / tw, image.get_height() as usize / th);
    let (gw, gh) = (gw, gh + ((gh / 2) * (sh / 2)));

    for gy in 0..gh {
        for gx in 0..gw {
            let is_darker_one = (gy + gx) % 2 == 0;

            let (ox, oy) = ((gx * tw + if gy % 2 == 0 { tw / 2 } else { 0 }) as isize, (gy * th) as isize);
            let (ox, oy) = (ox, oy - (gy as isize * sh as isize / 2));
            let (ox, oy) = (ox, oy + (sh as isize / 2));

            if ox < 0 || oy < 0 { continue; }
            let (ox, oy) = (ox as usize, oy as usize);

            fn reddify(x: usize, y: usize, image: &mut Image, darker: bool) {
                if x >= 8000 || y >= 3600 { return; }

                let mut pixel = image.get_pixel(x as u32, y as u32);
                pixel.r += min(u8::MAX - pixel.r, 255 / (if darker { 4 } else { 2 }));

                image.set_pixel(x as u32, y as u32, pixel);
            }

            fn line(x0: isize, y0: isize, x1: isize, y1: isize, image: &mut Image, darker: bool) {
                let (mut x0, mut y0) = (x0, y0);

                let dx = (x1 - x0).abs();
                let sx = if x0 < x1 { 1 } else { -1 };

                let dy = -(y1 - y0).abs();
                let sy = if y0 < y1 { 1 } else { -1 };

                let mut error = dx + dy;

                loop {
                    reddify(x0 as usize, y0 as usize, image, darker);

                    if x0 == x1 && y0 == y1 { break; };
                    let e2 = 2 * error;

                    if e2 >= dy {
                        if x0 == x1 { break; }

                        error = error + dy;
                        x0 = x0 + sx
                    }

                    if e2 <= dx {
                        if y0 == y1 { break; }
                        error = error + dx;
                        y0 = y0 + sy
                    }
                }
            }

            let (vx0, vy0) = (ox + tw / 2, oy);
            let (vx1, vy1) = (ox + tw, oy + (th - sh) / 2);
            let (vx2, vy2) = (ox + tw, oy + th - sh / 2);
            let (vx3, vy3) = (ox + tw / 2, oy + th);
            let (vx4, vy4) = (ox, oy + th - sh / 2);
            let (vx5, vy5) = (ox, oy + (th - sh) / 2);

            line(vx0 as isize, vy0 as isize, vx1 as isize, vy1 as isize, image, is_darker_one);
            line(vx1 as isize, vy1 as isize, vx2 as isize, vy2 as isize, image, is_darker_one);
            line(vx2 as isize, vy2 as isize, vx3 as isize, vy3 as isize, image, is_darker_one);
            line(vx3 as isize, vy3 as isize, vx4 as isize, vy4 as isize, image, is_darker_one);
            line(vx4 as isize, vy4 as isize, vx5 as isize, vy5 as isize, image, is_darker_one);
            line(vx5 as isize, vy5 as isize, vx0 as isize, vy0 as isize, image, is_darker_one);
        }
    }

    Ok(())
}