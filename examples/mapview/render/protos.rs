use libycresources::common::types::space::Elevation;
use libycresources::formats::map::blueprint;
use libycresources::formats::pal;
use libycresources::formats::pro::Type::{Critter, Item, Misc, Scenery, Wall};

use crate::cli::export::filter::Layers;
use crate::error::Error;
use crate::render::{frame, sprite};
use crate::traits::render::Provider;

pub(crate) fn imprint<'a, P: Provider>(
    protos: &Vec<blueprint::prototype::Instance>,
    provider: &P,
    elevation: &Elevation,
    palette: &pal::Palette,
    darkness: u8,
    layers: &Layers,
    image: &mut bmp::Image,
) -> Result<(), Error<'a>> {
    for proto in protos.iter() {
        if proto.id.kind == Item(()) && !(layers.items || layers.all()) { continue; };
        if proto.id.kind == Critter(()) && !(layers.critters || layers.all()) { continue; };
        if proto.id.kind == Scenery(()) && !(layers.scenery || layers.all()) { continue; };
        if proto.id.kind == Wall(()) && !(layers.walls || layers.all()) { continue; };
        if proto.id.kind == Misc(()) && !(layers.misc || layers.all()) { continue; };

        if let (
            Some(location),
            correction
        ) = (
            &proto.location.grid,
            &proto.location.screen.correction
        ) {
            if &location.elevation != elevation { continue; }

            let identifier = &proto.appearance.sprite;
            let item = provider.provide(&identifier)?;

            let (sprite, palette) = (item.0, item.1.as_ref().unwrap_or(palette));
            assert_eq!(location.orientation.scaled.scale.len(), sprite.orientations.len());

            let (frame, shift) = sprite::frame(
                &sprite, &location.orientation, proto.appearance.current,
            )?;

            let (tw, th) = (80isize, 36isize);
            let (tx, ty) = (location.position.x.value as isize, location.position.y.value as isize);

            let (ox, oy) = ((tx * tw) as isize, (ty * th) as isize);
            let (ox, oy) = (ox + (ty * 32), oy + ((location.position.x.scale.len() as isize - tx) * 12));
            let (ox, oy) = (ox - (tx * 32), oy - (ty * 12));
            let (ox, oy) = (ox / 2, oy / 2);

            let (ox, oy) = (
                ox - (frame.size.width as i16 + frame.shift.x) as isize / 2,
                oy - (frame.size.height as i16 + frame.shift.y) as isize
            );

            // Aligning with the hex grid from tiles' one.
            let (ox, oy) = (
                ox + (16) - if tx % 2 != 0 { 8 } else { 0 },
                oy + (16 + 8) - if tx % 2 != 0 { 6 } else { 0 }
            );

            let (ox, oy) = (
                ox + correction.x.value as isize,
                oy + correction.y.value as isize
            );

            let (ox, oy) = (
                ox + shift.x as isize,
                oy + shift.y as isize
            );

            frame::imprint(frame, palette, darkness, (ox, oy), image);
        }
    }

    Ok(())
}