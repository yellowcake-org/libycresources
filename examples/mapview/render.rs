use std::ops::Range;
use std::path::PathBuf;

use libycresources::common::types::errors::Error;
use libycresources::common::types::geometry::Coordinate;
use libycresources::common::types::models::Identifier;
use libycresources::common::types::models::sprite::Kind;
use libycresources::formats::frm::Sprite;
use libycresources::formats::map;
use libycresources::formats::map::tiles::Instance;
use libycresources::formats::pal::Palette;

use crate::Layers;
use crate::traits::RenderProvider;

mod fetch;
mod frame;

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

    let (tw, th, scale) = floors.first()
        .map(|t| { t.sprite.animations.first().map(|a| { (a, t.position) }) }).flatten()
        .map(|a| { a.0.frames.first().map(|f| { (f, a.1) }) }).flatten()
        .map(|f| { (f.0.size.width + f.0.shift.x, f.0.size.height + f.0.shift.y, f.1) })
        .map(|t| { (t.0 as usize, t.1 as usize, t.2.x.scale.len() as usize) })
        .ok_or(Error::Format)?;

    let (w, h) = (tw * scale, th * scale);
    let mut image = bmp::Image::new(w as u32, h as u32);

    for tile in floors.iter() {
        let palette = tile.palette
            .as_ref()
            .unwrap_or(&palette);

        let frame = tile.sprite
            .animations.first().ok_or(Error::Format)?
            .frames.first().ok_or(Error::Format)?;

        let (tx, ty) = (
            tile.position.x.value as usize * (scale / tile.position.x.scale.len()),
            tile.position.y.value as usize * (scale / tile.position.y.scale.len())
        );

        let (x, y) = (
            tw * tx,
            th * ty
        );

        let (x, y) = (
            x + (ty * (tw - 48)),
            y + ((scale - tx) * (th - 24))
        );

        let (x, y) = (
            x - (tx * 32),
            y - (ty * 12)
        );

        let (x, y) = (
            x + (tx * frame.shift.x as usize),
            y + (ty * frame.shift.y as usize)
        );

        frame::imprint(frame, palette, (x, y), &mut image);
    }

    Ok(image)
}