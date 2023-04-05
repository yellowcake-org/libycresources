use ycresources::common::types::geometry::{Orientation, Scaled};
use ycresources::common::types::models::Identifier;
use ycresources::common::types::models::sprite::Kind;
use ycresources::formats::map;
use ycresources::formats::pal::Palette;

use crate::error::Error;
use crate::render::{frame, grid, sprite};
use crate::render::item::Instance;
use crate::traits::render::Provider;

pub(crate) fn imprint<'a, 'b>(
    tiles: &'a Vec<Instance>,
    is_roof: bool,
    palette: &Palette,
    darkness: u8,
    image: &mut (&mut Vec<(u8, u8, u8)>, (usize, usize)),
) -> Result<(), Error<'b>> {
    let bounds = image.1;

    for tile in tiles.iter() {
        let palette = tile.palette.as_ref().unwrap_or(palette);
        let (frame, shift) = sprite::frame(
            &tile.sprite, &Orientation { scaled: Scaled { value: 0, scale: 0..6 } }, None,
        )?;

        let dimensions = (frame.size.width as usize, frame.size.height as usize);
        let point = grid::screen(tile.position, dimensions, bounds);

        let (x, y) = (point.x.value as isize, point.y.value as isize - if is_roof { 96 } else { 0 });
        let (x, y) = (x + shift.x as isize, y + shift.y as isize);

        frame::imprint(frame, palette, darkness, (x, y), image)?;
    }

    Ok(())
}

// TODO: Move to 'from' trait implementation?
pub(crate) fn convert<'a, P: Provider>(
    raw: &'a Vec<map::tiles::Instance<u8, u8>>,
    provider: &P,
) -> Result<Vec<Instance<'a>>, Error<'a>> {
    raw.iter()
        .map(|e| {
            let position = &e.position;
            let identifier = Identifier {
                kind: Kind::Tile,
                index: e.id & 0b1111_1111_1111,
            };

            let (sprite, palette) = provider.provide(&identifier)?;
            Ok(Instance { sprite, palette, position })
        })
        .collect()
}