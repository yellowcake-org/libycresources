pub struct Size {
    pub width: u16,
    pub hegith: u16,
}

pub struct Shift {
    pub x: u16,
    pub y: u16,
}

type ColorIndex = u8;

pub struct Frame {
    pub size: Size,
    pub pixels: Vec<ColorIndex>,
}

pub struct Animation {
    pub shift: Shift,
    pub frames: Vec<Frame>,
}

type FrameIndex = u16;
type AnimationIndex = u8;

pub struct Sprite {
    pub fps: u16,
    pub count: u16,

    pub keyframe: FrameIndex,
    pub animations: Vec<Animation>,

    pub orientations: [AnimationIndex; 6],
}
