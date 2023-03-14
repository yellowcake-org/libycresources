use item::Instance;
use libycresources::common::types::space::Elevation;
use libycresources::formats::map;
use libycresources::formats::pal::Palette;
use libycresources::formats::pro::meta::info::flags::Root::Flat;

use crate::cli::export::darkness::Darkness;
use crate::cli::export::filter::Layers;
use crate::error::Error;
use crate::traits::render::Provider;

mod frame;
mod tiles;
mod hexes;
mod protos;
mod item;
mod sprite;
mod grid;

pub(crate) fn map<'a, P: Provider>(
    map: &'a map::Map,
    layers: &Layers,
    darkness: Option<&Darkness>,
    elevation: &Elevation,
    provider: &P,
    palette: &Palette,
) -> Result<Option<(Vec<(u8, u8, u8)>, (usize, usize))>, Error<'a>> {
    let default = u8::try_from(map.darkness)
        .map_err(|_| Error::Corrupted("Map darkness value is out of range."))?;

    let darkness = darkness.map_or(
        default,
        |d| {
            match d {
                Darkness::None => 1,
                Darkness::Night => 2,
                Darkness::Dusk => 3,
                Darkness::Day => 4,
            }
        },
    );

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
        .map(|t| { (t.0 as usize, t.1 as usize, t.2.x.scale.len() as usize) })
        .ok_or(Error::Corrupted("Failed to determine tiles' grid parameters."))?;

    let (w, h) = (tw * scale, th * scale);
    let mut pixels = vec![(u8::MIN, u8::MIN, u8::MIN); w * h];
    let mut image = (&mut pixels, (w, h));

    if layers.floor || layers.all() {
        println!("Rendering floor...");
        tiles::imprint(&floors, false, palette, darkness, &mut image)?;
    }

    if layers.overlay || layers.all() {
        println!("Rendering hexagonal tiles' overlay...");
        hexes::overlay(&mut image)?;
    }

    println!("Rendering prototypes...");
    let (flat, normal) = map.prototypes.iter()
        .partition(|p| p.patch.meta.flags.contains(&Flat));

    protos::imprint(
        &flat,
        provider,
        &elevation,
        &palette,
        darkness,
        &layers,
        (tw, th),
        &mut image,
    )?;

    protos::imprint(
        &normal,
        provider,
        &elevation,
        &palette,
        darkness,
        &layers,
        (tw, th),
        &mut image,
    )?;

    if layers.roof || layers.all() {
        println!("Rendering roofs...");
        tiles::imprint(&ceilings, true, &palette, darkness, &mut image)?;
    }

    println!("Success.");
    Ok(Some((pixels, (w, h))))
}
