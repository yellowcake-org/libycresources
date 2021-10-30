pub mod parse;

pub struct Pixel {
    pub value: usize,
    pub scale: usize,
}

pub struct Glyph {
    pub width: usize,
    pub height: usize,

    pub pixels: Vec<Pixel>,
}

pub struct Spacing {
    pub vertical: usize,
    pub horizontal: usize,
}

pub struct Font {
    pub height: usize,
    pub spacing: Spacing,

    pub glyphs: [Glyph; 256],
}
