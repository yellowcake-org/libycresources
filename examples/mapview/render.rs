use std::ops::Range;
use std::path::PathBuf;

use libycresources::common;
use libycresources::common::graphics;
use libycresources::common::types::errors::Error;
use libycresources::common::types::geometry::Coordinate;
use libycresources::common::types::models::Identifier;
use libycresources::common::types::models::sprite::Kind;
use libycresources::formats::{frm, map};
use libycresources::formats::frm::parse::sprite;
use libycresources::formats::frm::Sprite;
use libycresources::formats::map::tiles::Instance;
use libycresources::formats::pal::Palette;

use crate::Layers;
use crate::traits::RenderProvider;

mod fetch;

pub(crate) fn map<P: RenderProvider>(
    map: &map::Map,
    filter: &Layers,
    provider: &P,
    resources: &PathBuf,
) -> Result<bmp::Image, Error> {
    let no_filter = !(filter.background ^ filter.tiles ^ filter.scenery ^ filter.walls);
    if no_filter { println!("Filter has not been applied, rendering all layers.") }

    println!("Loading COLOR.PAL...");
    let palette = fetch::palette(&resources.join("COLOR.PAL"))?;

    // TODO: Implement rendering all available elevations.
    let level = 0;

    let tiles = map.tiles
        .iter()
        .find(|e| { e.elevation.level.value == level })
        .ok_or(Error::Format)?;

    struct Tile<'a> {
        sprite: Sprite,
        palette: Option<Palette>,
        position: &'a Coordinate<u8, Range<u8>>,
    }

    fn convert<'a, P: RenderProvider>(raw: &'a Vec<Instance<u8, u8>>, provider: &P) -> Result<Vec<Tile<'a>>, Error> {
        raw.iter()
            .map(|e| {
                let identifier = Identifier { kind: Kind::Tile, value: e.id };

                let (sprite, palette) = provider.provide(&identifier)?;
                let position = &e.position;

                Ok(Tile { sprite, palette, position })
            })
            .collect()
    }

    let floors: Vec<Tile> = convert(&tiles.floor, provider)?;
    let ceilings: Vec<Tile> = convert(&tiles.ceiling, provider)?;

    let (tw, th) = floors.first()
        .map(|t| { t.sprite.animations.first() }).flatten()
        .map(|a| { a.frames.first() }).flatten()
        .map(|f| { (f.size.width + f.shift.x, f.size.height + f.shift.y) })
        .map(|t| { (t.0 as u64, t.1 as u64) })
        .ok_or(Error::Format)?;

    const ISOMETRIC_SIDE: u64 = 100;
    let (w, h) = (tw as u64 * ISOMETRIC_SIDE, th as u64 * ISOMETRIC_SIDE);
    let mut image = bmp::Image::new(w as u32, h as u32);

    for tile in floors.iter() {
        let palette = tile.palette
            .as_ref()
            .unwrap_or(&palette);

        let frame = tile.sprite
            .animations.first().ok_or(Error::Format)?
            .frames.first().ok_or(Error::Format)?;

        let (tx, ty) = (
            tile.position.x.value as u64 * (ISOMETRIC_SIDE / tile.position.x.scale.len() as u64),
            tile.position.y.value as u64 * (ISOMETRIC_SIDE / tile.position.y.scale.len() as u64)
        );

        let (x, y) = (
            tw * tx,
            th * ty
        );

        let (x, y) = (
            x + (ty * (tw - 48)),
            y + ((ISOMETRIC_SIDE - tx) * (th - 24))
        );

        let (x, y) = (
            x - (tx * 32),
            y - (ty * 12)
        );

        let (x, y) = (
            x + (tx * frame.shift.x as u64),
            y + (ty * frame.shift.y as u64)
        );

        for (number, &index) in frame.indexes.iter().enumerate() {
            // TODO: Extract this to a method / trait / etc.
            let color = &palette.colors[index as usize];
            let pixel = match color {
                None => None,
                Some(c) => {
                    let red = ((c.red.value as usize * (u8::MAX as usize + 1)) / c.red.scale.len()) as u8;
                    let green = ((c.green.value as usize * (u8::MAX as usize + 1)) / c.green.scale.len()) as u8;
                    let blue = ((c.blue.value as usize * (u8::MAX as usize + 1)) / c.blue.scale.len()) as u8;

                    Some(bmp::Pixel::new(red, green, blue))
                }
            };

            let (rx, ry) = (
                number as u64 % frame.size.width as u64,
                number as u64 / frame.size.width as u64
            );

            let (x, y) = (x + rx, y + ry);
            if let Some(pixel) = pixel {
                image.set_pixel(x as u32, y as u32, pixel);
            }
        }
    }

    Ok(image)
}