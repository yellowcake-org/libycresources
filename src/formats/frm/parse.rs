use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

use super::*;

#[derive(Debug)]
pub enum Error {
    Read(std::io::Error),
    Format,
    Source,
}

pub fn sprite<S: Read + Seek>(source: &mut S) -> Result<Sprite, Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(Error::Read(error));
    }

    let mut version_bytes = vec![0u8; 4];
    match source.read_exact(&mut version_bytes) {
        Err(error) => return Err(Error::Read(error)),
        Ok(value) => value,
    };

    let mut fps_bytes = vec![0u8; size_of::<u16>()];
    match source.read_exact(&mut fps_bytes) {
        Err(error) => return Err(Error::Read(error)),
        Ok(value) => value,
    };

    let fps = u16::from_be_bytes(match fps_bytes.try_into() {
        Err(_) => return Err(Error::Source),
        Ok(value) => value,
    });

    let mut keyframe_idx_bytes = vec![0u8; size_of::<u16>()];
    match source.read_exact(&mut keyframe_idx_bytes) {
        Err(error) => return Err(Error::Read(error)),
        Ok(value) => value,
    };

    let keyframe_idx = u16::from_be_bytes(match keyframe_idx_bytes.try_into() {
        Err(_) => return Err(Error::Source),
        Ok(value) => value,
    });

    let mut fpo_bytes = vec![0u8; size_of::<u16>()];
    match source.read_exact(&mut fpo_bytes) {
        Err(error) => return Err(Error::Read(error)),
        Ok(value) => value,
    };

    // frames per orientation
    let fpo = u16::from_be_bytes(match fpo_bytes.try_into() {
        Err(_) => return Err(Error::Source),
        Ok(value) => value,
    });

    let mut animation_shifts_x = [0u16; 6];
    let mut animation_shifts_y = [0u16; 6];

    for shift_x in &mut animation_shifts_x {
        let mut shift_x_bytes = vec![0u8; size_of::<u16>()];
        match source.read_exact(&mut shift_x_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        *shift_x = u16::from_be_bytes(match shift_x_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        });
    }

    for shift_y in &mut animation_shifts_y {
        let mut shift_y_bytes = vec![0u8; size_of::<u16>()];
        match source.read_exact(&mut shift_y_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        *shift_y = u16::from_be_bytes(match shift_y_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        });
    }

    let mut frame_offsets = [0u32; 6];

    for frame_offset in &mut frame_offsets {
        let mut frame_offset_bytes = vec![0u8; size_of::<u32>()];
        match source.read_exact(&mut frame_offset_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        *frame_offset = u32::from_be_bytes(match frame_offset_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        });
    }

    let mut framedata_len_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut framedata_len_bytes) {
        Err(error) => return Err(Error::Read(error)),
        Ok(value) => value,
    };

    let mut map: HashMap<u32, AnimationIndex> = HashMap::new();

    let mut animations: Vec<Animation> = Vec::new();
    let mut orientations = [0; 6];

    for orientation_idx in 0..6 {
        let offset = &frame_offsets[orientation_idx];

        match map.get(offset) {
            Some(animation_index) => {
                orientations[orientation_idx] = *animation_index;
            }
            None => {
                let mut frames: Vec<Frame> = Vec::new();

                for _ in 0..fpo {
                    let mut width_bytes = vec![0u8; size_of::<u16>()];
                    match source.read_exact(&mut width_bytes) {
                        Err(error) => return Err(Error::Read(error)),
                        Ok(value) => value,
                    };

                    let width = u16::from_be_bytes(match width_bytes.try_into() {
                        Err(_) => return Err(Error::Source),
                        Ok(value) => value,
                    });

                    let mut height_bytes = vec![0u8; size_of::<u16>()];
                    match source.read_exact(&mut height_bytes) {
                        Err(error) => return Err(Error::Read(error)),
                        Ok(value) => value,
                    };

                    let height = u16::from_be_bytes(match height_bytes.try_into() {
                        Err(_) => return Err(Error::Source),
                        Ok(value) => value,
                    });

                    let mut count_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut count_bytes) {
                        Err(error) => return Err(Error::Read(error)),
                        Ok(value) => value,
                    };

                    let mut shift_x_bytes = vec![0u8; size_of::<u16>()];
                    match source.read_exact(&mut shift_x_bytes) {
                        Err(error) => return Err(Error::Read(error)),
                        Ok(value) => value,
                    };

                    let shift_x = u16::from_be_bytes(match shift_x_bytes.try_into() {
                        Err(_) => return Err(Error::Source),
                        Ok(value) => value,
                    });

                    let mut shift_y_bytes = vec![0u8; size_of::<u16>()];
                    match source.read_exact(&mut shift_y_bytes) {
                        Err(error) => return Err(Error::Read(error)),
                        Ok(value) => value,
                    };

                    let shift_y = u16::from_be_bytes(match shift_y_bytes.try_into() {
                        Err(_) => return Err(Error::Source),
                        Ok(value) => value,
                    });

                    let square = (width as u32 * height as u32) as usize;
                    let mut pixels = vec![0u8; square * size_of::<u8>()];
                    match source.read_exact(&mut pixels) {
                        Err(error) => return Err(Error::Read(error)),
                        Ok(value) => value,
                    };

                    frames.push(Frame {
                        size: Size { width, height },
                        shift: Shift {
                            x: shift_x,
                            y: shift_y,
                        },
                        pixels,
                    });
                }

                let animation = Animation {
                    shift: Shift {
                        x: animation_shifts_x[orientation_idx],
                        y: animation_shifts_y[orientation_idx],
                    },
                    frames,
                };

                animations.push(animation);
                map.insert(*offset, (animations.len() - 1) as AnimationIndex);
            }
        }
    }

    Ok(Sprite {
        fps,
        count: fpo,
        keyframe: keyframe_idx as FrameIndex,
        animations,
        orientations,
    })
}
