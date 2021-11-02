use libycresources::formats::aaf;

pub fn glyph(glyph: &aaf::Glyph) {
    if !glyph.pixels.is_empty() {
        for row in 0..glyph.height {
            for column in 0..glyph.width {
                let pixel = &glyph.pixels[row * glyph.width + column];
                let levels = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

                print!("{:}", levels[levels.len() * pixel.value / pixel.scale])
            }

            if row != glyph.height {
                println!();
            }
        }
    }
}
