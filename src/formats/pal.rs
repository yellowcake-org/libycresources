pub mod parse;

pub struct Palette {
    pub colors: [Option<crate::common::graphics::ColorPixel>; 256],
}
