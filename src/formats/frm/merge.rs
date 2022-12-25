use super::*;

#[derive(Debug)]
pub enum Error { Inconsistency }

pub fn sprites(source: [Sprite; 6]) -> Result<Sprite, Error> {
    let fps = source[0].fps;
    let count = source[0].count;
    let keyframe = source[0].keyframe;
    let animations_len = 1;

    let is_fps_consistent = source.iter().all(|next| next.fps == fps);
    let is_count_consistent = source.iter().all(|next| next.count == count);
    let is_keyframe_consistent = source.iter().all(|next| next.keyframe == keyframe);
    let is_animations_len_consistent = source
        .iter()
        .all(|next| next.animations.len() == animations_len);

    let is_source_consistent = is_fps_consistent
        && is_count_consistent
        && is_keyframe_consistent
        && is_animations_len_consistent;

    if !is_source_consistent {
        return Err(Error::Inconsistency);
    }

    let mut animations = Vec::new();

    for mut sprite in source {
        animations.push(sprite.animations.remove(0));
    }

    Ok(Sprite {
        fps,
        count,
        keyframe,
        animations,
        orientations: [0, 1, 2, 3, 4, 5],
    })
}
