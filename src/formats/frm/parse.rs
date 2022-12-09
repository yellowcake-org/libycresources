use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::common::types::errors::Error;

use super::*;

pub fn sprite<S: Read + Seek>(source: &mut S) -> Result<Sprite, Error> {
    const ORIENTATIONS_COUNT: usize = 6;

    source.seek(SeekFrom::Start(0))?;
    source.seek(SeekFrom::Current(4))?;

    let fps = source.read_u16::<BigEndian>()?;
    let keyframe_idx = source.read_u16::<BigEndian>()?;
    let fpo = source.read_u16::<BigEndian>()?;

    let mut animation_shifts_x = [i16::MIN; ORIENTATIONS_COUNT];
    for shift_x in &mut animation_shifts_x {
        *shift_x = source.read_i16::<BigEndian>()?;
    }

    let mut animation_shifts_y = [i16::MIN; ORIENTATIONS_COUNT];
    for shift_y in &mut animation_shifts_y {
        *shift_y = source.read_i16::<BigEndian>()?;
    }

    let mut frame_offsets = [u32::MIN; ORIENTATIONS_COUNT];
    for frame_offset in &mut frame_offsets {
        *frame_offset = source.read_u32::<BigEndian>()?;
    }

    source.seek(SeekFrom::Current(4))?;

    let mut map: HashMap<u32, AnimationIndex> = HashMap::new();
    let mut animations: Vec<Animation> = Vec::new();
    let mut orientations = [AnimationIndex::MIN; ORIENTATIONS_COUNT];

    for orientation_idx in usize::MIN..orientations.len() {
        let offset = &frame_offsets[orientation_idx];

        match map.get(offset) {
            Some(animation_index) => {
                orientations[orientation_idx] = *animation_index;
            }
            None => {
                let mut frames: Vec<Frame> = Vec::new();

                for _ in 0..fpo {
                    let width = source.read_u16::<BigEndian>()?;
                    let height = source.read_u16::<BigEndian>()?;

                    source.seek(SeekFrom::Current(4))?;

                    let x = source.read_u16::<BigEndian>()?;
                    let y = source.read_u16::<BigEndian>()?;

                    let square = (width as u32 * height as u32) as usize;

                    let mut pixels = vec![u8::MIN; square];
                    source.read_exact(&mut pixels)?;

                    frames.push(Frame {
                        size: Size { width, height },
                        shift: Shift {
                            x: i16::try_from(x).map_err(|_| { Error::Format })?,
                            y: i16::try_from(y).map_err(|_| { Error::Format })?
                        },
                        indexes: pixels,
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
