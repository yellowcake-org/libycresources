use std::path::PathBuf;

use libycresources::common::types::errors::Error;
use libycresources::formats::map;

use crate::Layers;
use crate::traits::RenderProvider;

mod fetch;

pub(crate) fn map<P: RenderProvider>(
    _map: &map::Map,
    filter: &Layers,
    _provider: &P,
    resources: &PathBuf,
) -> Result<bmp::Image, Error> {
    let no_filter = !(filter.background ^ filter.tiles ^ filter.scenery ^ filter.walls);
    if no_filter { println!("Filter has not been applied, rendering all layers.") }

    println!("Loading COLOR.PAL...");
    let _palette = fetch::palette(&resources.join("COLOR.PAL"))?;

    return Ok(bmp::Image::new(1, 1));
}