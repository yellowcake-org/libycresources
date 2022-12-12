use libycresources::common::types::errors::Error;
use libycresources::common::types::geometry::Orientation;
use libycresources::formats::frm::{Frame, FrameIndex, Shift, Sprite};

pub(crate) fn frame<'a>(
    sprite: &'a Sprite, orientation: &Orientation, index: Option<FrameIndex>,
) -> Result<(&'a Frame, &'a Shift), Error> {
    let orientation_idx = orientation.scaled.value as usize * 6 / orientation.scaled.scale.len();
    let animation_idx = sprite.orientations[orientation_idx];

    let animation = sprite.animations
        .get(animation_idx as usize)
        .ok_or(Error::Format)?;

    let frame_idx = index.unwrap_or(sprite.keyframe);
    animation.frames.get(frame_idx as usize).ok_or(Error::Format).map(|f| { (f, &animation.shift) })
}