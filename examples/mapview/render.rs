use std::fs::File;
use std::path::PathBuf;

use item::Instance;
use libycresources::common::types::errors::Error;
use libycresources::formats::{map, pal};

use crate::Layers;
use crate::traits::render::Provider;

mod frame;
mod tiles;
mod hexes;
mod protos;
mod item;

pub(crate) fn map<P: Provider>(
    map: &map::Map,
    filter: &Layers,
    provider: &P,
    resources: &PathBuf,
) -> Result<bmp::Image, Error> {
    let no_filter = !(
        filter.floor ^ filter.overlay ^ filter.roof ^ filter.scenery ^ filter.walls ^ filter.creatures
    );

    if no_filter { println!("Filter has not been applied, rendering all layers.") }

    let file = File::open(&resources.join("COLOR.PAL"))?;
    let mut reader = std::io::BufReader::with_capacity(1 * 1024 * 1024, file);

    let palette = pal::parse::palette(&mut reader)?;

    let level = 0;
    let tiles = map.tiles
        .iter()
        .find(|e| { e.elevation.level.value == level })
        .ok_or(Error::Format)?;

    let floors: Vec<Instance> = tiles::convert(&tiles.floor, provider)?;
    let ceilings: Vec<Instance> = tiles::convert(&tiles.ceiling, provider)?;

    let (tw, th, scale) = floors.first()
        .map(|t| { t.sprite.animations.first().map(|a| { (a, t.position) }) }).flatten()
        .map(|a| { a.0.frames.first().map(|f| { (f, a.1) }) }).flatten()
        .map(|f| { (f.0.size.width + f.0.shift.x, f.0.size.height + f.0.shift.y, f.1) })
        .map(|t| { (t.0 as usize, t.1 as usize, t.2.x.scale.len() as usize) }).ok_or(Error::Format)?;

    let (w, h) = (tw * scale, th * scale);
    let mut image = bmp::Image::new(w as u32, h as u32);

    if filter.floor { tiles::imprint(&floors, &palette, scale, &mut image)?; }
    if filter.overlay { hexes::overlay(&mut image)?; }
    if filter.roof { tiles::imprint(&ceilings, &palette, scale, &mut image)?; }

    Ok(image)
}
