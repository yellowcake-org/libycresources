use libycresources::formats::map;

use crate::Layers;

pub(crate) fn map(map: &map::Map, filter: &Layers) -> bmp::Image {
    let no_filter = !(filter.background ^ filter.tiles ^ filter.scenery ^ filter.walls);
    if no_filter { println!("Filter has not been applied, rendering all layers.") }

    return bmp::Image::new(1, 1);
}