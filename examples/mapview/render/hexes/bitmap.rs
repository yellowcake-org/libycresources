use std::cmp::min;

// Bresenham's line algorithm
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
pub(crate) fn line(
    start: (usize, usize), end: (usize, usize), image: &mut (&mut Vec<(u8, u8, u8)>, (usize, usize)),
) {
    let mut current = start;

    let dx: isize = (end.0 as isize - current.0 as isize).abs();
    let sx: isize = if current.0 < end.0 { 1 } else { -1 };

    let dy: isize = -(end.1 as isize - current.1 as isize).abs();
    let sy: isize = if current.1 < end.1 { 1 } else { -1 };

    let mut error = dx + dy;

    loop {
        fn reddify(x: usize, y: usize, image: &mut (&mut Vec<(u8, u8, u8)>, (usize, usize))) {
            let index = x + (y * image.1.0);

            if let Some(pixel) = image.0.get(index).map(|pixel| {
                (pixel.0 + min(u8::MAX - pixel.0, 255 / 4), pixel.1, pixel.2)
            }) { image.0[index] = pixel; } else { debug_assert!(true) };
        }

        reddify(current.0 as usize, current.1 as usize, image);
        if current.0 == end.0 && current.1 == end.1 { break; };

        let error_doubled = 2 * error;
        if error_doubled >= dy {
            if current.0 == end.0 { break; }

            error = error + dy;
            current.0 = (current.0 as isize + sx) as usize
        }

        if error_doubled <= dx {
            if current.1 == end.1 { break; }

            error = error + dx;
            current.1 = (current.1 as isize + sy) as usize
        }
    }
}