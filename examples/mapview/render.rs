use std::fs::File;
use std::path::PathBuf;

use item::Instance;
use libycresources::common::types::errors::Error;
use libycresources::common::types::space::Elevation;
use libycresources::formats::{map, pal};

use crate::cli::export::darkness::Darkness;
use crate::cli::export::filter::Layers;
use crate::traits::render::Provider;

mod frame;
mod tiles;
mod hexes;
mod protos;
mod item;
mod sprite;

pub(crate) fn map<P: Provider>(
    map: &map::Map,
    layers: &Layers,
    darkness: Option<&Darkness>,
    elevation: &Elevation,
    provider: &P,
    resources: &PathBuf,
) -> Result<Option<bmp::Image>, Error> {
    let darkness = darkness
        .map_or(u8::try_from(map.darkness).map_err(|_| Error::Format)?, |d| {
            match d {
                Darkness::None => 1,
                Darkness::Night => 2,
                Darkness::Dusk => 3,
                Darkness::Day => 4,
            }
        });

    let tiles = map.tiles
        .iter()
        .find(|e| { &e.elevation == elevation });

    let tiles = match tiles {
        None => return Ok(None),
        Some(value) => value
    };

    let floors: Vec<Instance> = tiles::convert(&tiles.floor, provider)?;
    let ceilings: Vec<Instance> = tiles::convert(&tiles.ceiling, provider)?;

    let (tw, th, scale) = floors.first()
        .map(|t| { t.sprite.animations.first().map(|a| { (a, t.position) }) }).flatten()
        .map(|a| { a.0.frames.first().map(|f| { (f, a.1) }) }).flatten()
        .map(|f| { (f.0.size.width as i16 + f.0.shift.x, f.0.size.height as i16 + f.0.shift.y, f.1) })
        .map(|t| { (t.0 as usize, t.1 as usize, t.2.x.scale.len() as usize) }).ok_or(Error::Format)?;

    let (w, h) = (tw * scale, th * scale);
    let mut image = bmp::Image::new(w as u32, h as u32);

    let file = File::open(&resources.join("COLOR.PAL"))?;
    let mut reader = std::io::BufReader::with_capacity(1 * 1024 * 1024, file);
    let palette = pal::parse::palette(&mut reader)?;

    if layers.floor || layers.all() {
        println!("Rendering floor...");
        tiles::imprint(&floors, false, &palette, darkness, scale, &mut image)?;
    }

    if layers.overlay || layers.all() {
        println!("Rendering hexagonal tiles' overlay...");
        hexes::overlay(&mut image)?;
    }

    println!("Rendering prototypes...");
    protos::imprint(&map.prototypes, provider, &elevation, &palette, darkness, &layers, &mut image)?;

    if layers.roof || layers.all() {
        println!("Rendering roofs...");
        tiles::imprint(&ceilings, true, &palette, darkness, scale, &mut image)?;
    }

    println!("Success.");
    Ok(Some(image))
}
