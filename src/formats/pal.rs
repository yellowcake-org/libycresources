use crate::common::graphics::Pixel;

pub mod parse;

pub struct Palette {
    pub colors: [Option<Pixel>; 256],
}
