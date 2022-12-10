use bmp::Image;

use libycresources::common::types::errors::Error;
use libycresources::common::types::models::Identifier;
use libycresources::common::types::models::sprite::Kind;
use libycresources::formats::map;
use libycresources::formats::pal::Palette;

use crate::render::frame;
use crate::render::item::Instance;
use crate::traits::render::Provider;

pub(crate) fn imprint(
    tiles: &Vec<Instance>,
    palette: &Palette,
    side: usize,
    image: &mut Image,
) -> Result<(), Error> {
    for tile in tiles.iter() {
        let palette = tile.palette.as_ref().unwrap_or(palette);

        let animation = tile.sprite.animations.first().ok_or(Error::Format)?;
        let frame = animation.frames.first().ok_or(Error::Format)?;

        let (tw, th) = (frame.size.width as usize, frame.size.height as usize);
        let (tx, ty) = (
            tile.position.x.value as usize * (side / tile.position.x.scale.len()),
            tile.position.y.value as usize * (side / tile.position.y.scale.len())
        );

        let (x, y) = (tw * tx, th * ty);
        let (x, y) = (x + (ty * 32), y + ((side - tx) * 12));
        let (x, y) = (x - (tx * 32), y - (ty * 12));

        let (x, y) = (x as i16 + animation.shift.x, y as i16 + animation.shift.y);
        let (x, y) = (x as usize, y as usize);

        frame::imprint(frame, palette, (x, y), image);
    }

    Ok(())
}

// TODO: Move to 'from' trait implementation?
pub(crate) fn convert<'a, P: Provider>(
    raw: &'a Vec<map::tiles::Instance<u8, u8>>,
    provider: &P,
) -> Result<Vec<Instance<'a>>, Error> {
    raw.iter()
        .map(|e| {
            let identifier = Identifier { kind: Kind::Tile, value: e.id };

            let (sprite, palette) = provider.provide(&identifier)?;
            let position = &e.position;

            Ok(Instance { sprite, palette, position })
        })
        .collect()
}