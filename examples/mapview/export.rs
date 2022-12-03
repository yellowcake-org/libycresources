use libycresources::formats::map;

use crate::Layers;

pub(crate) fn export(map: &map::Map, filter: &Layers) {
    let no_filter = !(filter.background ^ filter.tiles ^ filter.scenery ^ filter.walls);
    if no_filter { println!("Filter has not been applied, rendering all layers.") }
}