pub mod merge;
pub mod parse;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Shift {
    pub x: u16,
    pub y: u16,
}

pub type ColorIndex = u8;

pub struct Frame {
    pub size: Size,
    pub shift: Shift,
    pub pixels: Vec<ColorIndex>,
}

pub struct Animation {
    pub shift: Shift,
    pub frames: Vec<Frame>,
}

pub type FrameIndex = u16;
pub type AnimationIndex = u8;

pub struct Sprite {
    pub fps: u16,
    pub count: u16,

    pub keyframe: FrameIndex,
    pub animations: Vec<Animation>,

    pub orientations: [AnimationIndex; 6],
}
