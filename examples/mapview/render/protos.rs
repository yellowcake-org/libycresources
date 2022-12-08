use libycresources::common::types::errors::Error;
use libycresources::formats::map::blueprint;
use libycresources::formats::pal;

use crate::Layers;
use crate::render::frame;

pub(crate) fn imprint(
    protos: &Vec<blueprint::prototype::Instance>,
    palette: &pal::Palette,
    include: &Layers,
    image: &mut bmp::Image,
) -> Result<(), Error> {
    for proto in protos.iter() {
        todo!()
    }

    Ok(())
}