use std::ops::Range;

use bmp::Image;

use libycresources::common::types::errors::Error;
use libycresources::common::types::geometry::Coordinate;
use libycresources::common::types::models::Identifier;
use libycresources::common::types::models::sprite::Kind;
use libycresources::formats::frm::Sprite;
use libycresources::formats::map;
use libycresources::formats::pal::Palette;

use crate::render::frame;
use crate::traits::RenderProvider;

pub(crate) struct RenderTile<'a> {
    pub(crate) sprite: Sprite,
    pub(crate) palette: Option<Palette>,
    pub(crate) position: &'a Coordinate<u8, Range<u8>>,
}

pub(crate) fn imprint(
    tiles: &Vec<RenderTile>,
    palette: &Palette,
    scale: usize,
    image: &mut Image,
) -> Result<(), Error> {
    for tile in tiles.iter() {
        let palette = tile.palette.as_ref().unwrap_or(palette);

        let frame = tile.sprite
            .animations.first().ok_or(Error::Format)?
            .frames.first().ok_or(Error::Format)?;

        let (tw, th) = (
            frame.size.width + frame.shift.x,
            frame.size.height + frame.shift.y,
        );

        let (tw, th) = (tw as usize, th as usize);
        let (tx, ty) = (
            tile.position.x.value as usize * (scale / tile.position.x.scale.len()),
            tile.position.y.value as usize * (scale / tile.position.y.scale.len())
        );

        let (x, y) = (tw * tx, th * ty);
        let (x, y) = (x + (ty * (tw - 48)), y + ((scale - tx) * (th - 24)));
        let (x, y) = (x - (tx * 32), y - (ty * 12));
        let (x, y) = (x + (tx * frame.shift.x as usize), y + (ty * frame.shift.y as usize));

        frame::imprint(frame, palette, (x, y), image);
    }

    Ok(())
}

pub(crate) fn convert<'a, P: RenderProvider>(
    raw: &'a Vec<map::tiles::Instance<u8, u8>>,
    provider: &P,
) -> Result<Vec<RenderTile<'a>>, Error> {
    raw.iter()
        .map(|e| {
            let identifier = Identifier { kind: Kind::Tile, value: e.id };

            let (sprite, palette) = provider.provide(&identifier)?;
            let position = &e.position;

            Ok(RenderTile { sprite, palette, position })
        })
        .collect()
}