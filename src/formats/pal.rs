pub mod parse;

use crate::common::graphics::Pixel;

pub struct Palette {
    pub colors: [Option<Pixel>; 256],
}
