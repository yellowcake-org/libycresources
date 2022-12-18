use std::collections::HashMap;

use libycresources::common::types::space::Elevation;
use libycresources::formats::{pal, pro};
use libycresources::formats::map::blueprint;
use libycresources::formats::pro::Type::{Critter, Item, Misc, Scenery, Tile, Wall};

use crate::cli::export::filter::Layers;
use crate::error::Error;
use crate::render::{frame, grid, sprite};
use crate::traits::render::Provider;

pub(crate) fn imprint<'a, P: Provider>(
    protos: &Vec<&blueprint::prototype::Instance>,
    provider: &P,
    elevation: &Elevation,
    palette: &pal::Palette,
    darkness: u8,
    layers: &Layers,
    dimensions: (usize, usize),
    image: &mut (&mut Vec<(u8, u8, u8)>, (usize, usize)),
) -> Result<(), Error<'a>> {
    let bounds = image.1;

    let mut grid: HashMap<(u8, u8), Vec<&blueprint::prototype::Instance>> = HashMap::new();
    for proto in protos.iter() {
        if let Some(location) = &proto.location.grid {
            let location = (location.position.x.value, location.position.y.value);

            let mut new: Vec<&blueprint::prototype::Instance> = Vec::new();
            let existed = grid.get_mut(&location);

            let vec = existed.unwrap_or(&mut new);
            vec.push(proto);

            if !new.is_empty() { grid.insert(location, new); };
        }
    }

    for y in 0u8..200 {
        for x in 0u8..200 {
            if let Some(protos) = grid.get_mut(&(200 - x - 1, y)) {
                protos.sort_by(|l, r| {
                    fn weight<I, C, S, W, T, M>(t: &pro::Type<I, C, S, W, T, M>) -> u8 {
                        match t {
                            Item(_) => 4,
                            Critter(_) => 5,
                            Scenery(_) => 2,
                            Wall(_) => 1,
                            Tile(_) => 0,
                            Misc(_) => 3,
                        }
                    }

                    weight(&l.id.kind).cmp(&weight(&r.id.kind))
                });

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

                        let point = grid::screen(&location.position, dimensions, bounds);

                        // Aligning with frame's shift within it's own bounds.
                        let (x, y) = (
                            point.x.value as isize - (frame.size.width as i16 + frame.shift.x) as isize / 2,
                            point.y.value as isize - (frame.size.height as i16 + frame.shift.y) as isize
                        );

                        // Aligning with the hex grid from tiles' one.
                        let is_odd_row = location.position.x.value as isize % 2 != 0;
                        let (ox, oy) = (
                            x + (16) - if is_odd_row { 8 } else { 0 },
                            y + (16 + 8) - if is_odd_row { 6 } else { 0 }
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
            }
        }
    }

    Ok(())
}