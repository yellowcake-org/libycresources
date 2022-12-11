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
        let frame_idx = tile.sprite.keyframe;

        let animation = tile.sprite.animations.first().ok_or(Error::Format)?;
        let frame = animation.frames.get(frame_idx as usize).ok_or(Error::Format)?;

        let (tw, th) = (frame.size.width as isize, frame.size.height as isize);
        let (tx, ty) = (
            tile.position.x.value as isize * (side as isize / tile.position.x.scale.len() as isize),
            tile.position.y.value as isize * (side as isize / tile.position.y.scale.len() as isize)
        );

        let (x, y) = (tw * tx, th * ty);
        let (x, y) = (x + (ty * 32), y + ((side as isize - tx) * 12));
        let (x, y) = (x - (tx * 32), y - (ty * 12));

        let (x, y) = (x + animation.shift.x as isize, y + animation.shift.y as isize);

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
            let position = &e.position;
            let identifier = Identifier { raw: e.index as u32, kind: Kind::Tile, index: e.index };

            let (sprite, palette) = provider.provide(&identifier)?;
            Ok(Instance { sprite, palette, position })
        })
        .collect()
}