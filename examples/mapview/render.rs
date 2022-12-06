use std::path::PathBuf;

use libycresources::common::types::errors::Error;
use libycresources::formats::map;

use crate::Layers;
use crate::render::tiles::RenderTile;
use crate::traits::RenderProvider;

mod fetch;
mod frame;
mod tiles;

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

    let floors: Vec<RenderTile> = tiles::convert(&tiles.floor, provider)?;
    let ceilings: Vec<RenderTile> = tiles::convert(&tiles.ceiling, provider)?;

    // TODO: Figure out better way to know output parameters, or check if all tiles are consistent between each other.
    let (tw, th, scale) = floors.first()
        .map(|t| { t.sprite.animations.first().map(|a| { (a, t.position) }) }).flatten()
        .map(|a| { a.0.frames.first().map(|f| { (f, a.1) }) }).flatten()
        .map(|f| { (f.0.size.width + f.0.shift.x, f.0.size.height + f.0.shift.y, f.1) })
        .map(|t| { (t.0 as usize, t.1 as usize, t.2.x.scale.len() as usize) }).ok_or(Error::Format)?;

    let (w, h) = (tw * scale, th * scale);
    let mut image = bmp::Image::new(w as u32, h as u32);

    tiles::imprint(&floors, &palette, scale, &mut image)?;
    tiles::imprint(&ceilings, &palette, scale, &mut image)?;

    Ok(image)
}