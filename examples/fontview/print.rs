use ycresources::formats::aaf;

pub fn glyph(glyph: &aaf::Glyph) {
    if !glyph.dots.is_empty() {
        for row in 0..glyph.height {
            for column in 0..glyph.width {
                let index = (row * glyph.width + column) as usize;

                if let Some(pixel) = &glyph.dots.get(index) {
                    let levels = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
                    let index = levels.len() * pixel.value / (pixel.scale.end - pixel.scale.start);

                    if let Some(level) = levels.get(index) {
                        print!("{:}", level);
                    } else {
                        eprintln!("Encountered a pixel, which brightness level is of bounds.")
                    }
                } else {
                    eprintln!("Encountered a pixel located out of bounds.")
                }
            }

            if row != glyph.height { println!(); }
        }
    }
}
