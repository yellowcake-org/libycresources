use std::ops::Range;

use libycresources::common::types::geometry::{Coordinate, Scaled};

pub(crate) fn screen(
    from: &Coordinate<u8, Range<u8>>,
    dimensions: (usize, usize),
    bounds: (usize, usize),
) -> Coordinate<usize, Range<usize>> {
    // vanilla game grid size for floor / roofs
    const REF_SIDE_LEN: u8 = 100;

    // vanilla game tile size
    const REF_TILE_WIDTH: u8 = 80;
    const REF_TILE_HEIGHT: u8 = 36;

    let x_scale = from.x.scale.len() as isize / REF_SIDE_LEN as isize;
    let y_scale = from.y.scale.len() as isize / REF_SIDE_LEN as isize;

    let (tw, th) = (dimensions.0 as isize, dimensions.1 as isize);
    let (tx, ty) = (from.x.value as isize, from.y.value as isize);

    let x_overlap = 2f64 * REF_TILE_HEIGHT as f64 - (REF_TILE_WIDTH as f64 / 2f64);
    let y_overlap = (3f64 * REF_TILE_WIDTH as f64 - 4f64 * REF_TILE_HEIGHT as f64) / 8f64;

    let x_overlap = x_overlap as isize;
    let y_overlap = y_overlap as isize;

    let (x, y) = (tw * tx, th * ty);
    let (x, y) = (x + (ty * x_overlap), y + ((from.x.scale.len() as isize - tx) * y_overlap));
    let (x, y) = (x - (tx * x_overlap), y - (ty * y_overlap));
    let (x, y) = (x / x_scale, y / y_scale);
    let (x, y) = (x, y - y_overlap);

    Coordinate {
        x: Scaled { value: x as usize, scale: 0..bounds.0 },
        y: Scaled { value: y as usize, scale: 0..bounds.1 },
    }
}