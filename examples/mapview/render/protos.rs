use libycresources::common::types::errors::Error;
use libycresources::common::types::space::Elevation;
use libycresources::formats::map::blueprint;
use libycresources::formats::pal;
use libycresources::formats::pro::Type::{Critter, Item, Misc, Scenery, Wall};

use crate::Layers;
use crate::render::frame;
use crate::traits::render::Provider;

pub(crate) fn imprint<P: Provider>(
    protos: &Vec<blueprint::prototype::Instance>,
    provider: &P,
    elevation: &Elevation,
    palette: &pal::Palette,
    filter: &Layers,
    image: &mut bmp::Image,
) -> Result<(), Error> {
    for proto in protos.iter() {
        if proto.id.kind == Item(()) && !filter.items { continue; };
        if proto.id.kind == Critter(()) { continue; };
        if proto.id.kind == Scenery(()) && !filter.scenery { continue; };
        if proto.id.kind == Wall(()) && !filter.walls { continue; };
        if proto.id.kind == Misc(()) && !filter.misc { continue; };

        if let (
            Some(location),
            correction
        ) = (
            &proto.location.grid,
            &proto.location.screen.correction
        ) {
            if &location.elevation != elevation { continue; }

            let item = provider.provide(&proto.appearance.sprite)?;
            let (sprite, palette) = (item.0, item.1.as_ref().unwrap_or(palette));

            assert_eq!(location.orientation.scaled.scale.len(), sprite.orientations.len());

            let orientation_idx = location.orientation.scaled.value;
            let frame_idx = proto.appearance.current.unwrap_or(0);

            let fetched = sprite.animations
                .get(orientation_idx as usize)
                .map(|a| { a.frames.get(frame_idx as usize).map(|f| { (f, &a.shift) }) })
                .flatten();

            if let Some((frame, shift)) = fetched {
                let (tw, th) = (80usize, 36usize);

                let (tx, ty) = (
                    location.position.x.value as usize,
                    location.position.y.value as usize,
                );

                let (tx, ty) = (tx, ty);
                let (ox, oy) = ((tx * tw) as usize, (ty * th) as usize);

                let (ox, oy) = (ox + (ty * 32), oy + ((location.position.x.scale.len() - tx) * 12));
                let (ox, oy) = (ox - (tx * 32), oy - (ty * 12));
                let (ox, oy) = (ox / 2, oy / 2);

                // let (ox, oy) = (
                //     ox - (frame.size.width as i16 + frame.shift.x) as usize / 2,
                //     oy - (frame.size.height as i16 + frame.shift.y) as usize / 2
                // );

                let (ox, oy) = (
                    ox + correction.x.value as usize,
                    oy + correction.y.value as usize
                );

                let (ox, oy) = (
                    ox as isize + shift.x as isize,
                    oy as isize + shift.y as isize
                );

                let (ox, oy) = (ox as usize, oy as usize);
                frame::imprint(frame, palette, (ox, oy), image);
            }
        }
    }

    Ok(())
}