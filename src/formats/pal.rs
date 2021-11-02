pub mod parse;

use super::super::common::graphics::ColorPixel;

pub struct Palette {
    pub colors: [Option<ColorPixel>; 256],
}
