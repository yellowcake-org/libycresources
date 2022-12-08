use std::ops::Range;

use libycresources::common::types::geometry::Coordinate;
use libycresources::formats::frm::Sprite;
use libycresources::formats::pal::Palette;

pub(crate) struct Instance<'a> {
    pub(crate) sprite: Sprite,
    pub(crate) palette: Option<Palette>,
    pub(crate) position: &'a Coordinate<u8, Range<u8>>,
}
