use std::ops::Range;

use ycresources::common::types::geometry::Coordinate;
use ycresources::formats::frm::Sprite;
use ycresources::formats::pal::Palette;

pub(crate) struct Instance<'a> {
    pub(crate) sprite: Sprite,
    pub(crate) palette: Option<Palette>,
    pub(crate) position: &'a Coordinate<u8, Range<u8>>,
}
