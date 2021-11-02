use super::{ColorPixel, Pixel};

impl Copy for Pixel {}
impl Clone for Pixel {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            scale: self.scale,
        }
    }
}

impl Copy for ColorPixel {}
impl Clone for ColorPixel {
    fn clone(&self) -> Self {
        Self {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
}
