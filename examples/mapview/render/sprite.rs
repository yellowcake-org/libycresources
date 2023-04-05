use ycresources::common::types::geometry::Orientation;
use ycresources::formats::frm::{Frame, FrameIndex, Shift, Sprite};

use crate::error::Error;

pub(crate) fn frame<'a, 'b>(
    sprite: &'a Sprite, orientation: &Orientation, index: Option<FrameIndex>,
) -> Result<(&'a Frame, &'a Shift), Error<'b>> {
    let orientation_idx = orientation.scaled.value as usize * 6 / orientation.scaled.scale.len();
    let animation_idx = sprite.orientations[orientation_idx];

    let animation = sprite.animations
        .get(animation_idx as usize)
        .ok_or(Error::Corrupted("Failed to acquire animation at expected index."))?;

    let frame_idx = index.unwrap_or(sprite.keyframe);

    animation.frames.get(frame_idx as usize)
        .ok_or(Error::Corrupted("Failed to acquire frame at expected index."))
        .map(|f| { (f, &animation.shift) })
}