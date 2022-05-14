use crate::common::types::ScaledValue;

use super::super::*;

impl TryFrom<u32> for common::Coordinate {
    type Error = parse::errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        const SCALE: std::ops::Range<u8> = 0u8..200;
        if (0..(SCALE.end as u32).pow(2)).contains(&value) {
            let x = value / SCALE.end as u32;
            let y = value - (x * SCALE.end as u32);

            Ok(Self {
                x: ScaledValue { value: x as u8, scale: SCALE },
                y: ScaledValue { value: y as u8, scale: SCALE },
            })
        } else {
            return Err(parse::errors::Error::Format(parse::errors::Format::Data));
        }
    }
}

impl TryFrom<u32> for common::Elevation {
    type Error = parse::errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (0..3).contains(&value) {
            Ok(Self { level: ScaledValue { value: value as u8, scale: 0u8..3 } })
        } else {
            return Err(parse::errors::Error::Format(parse::errors::Format::Data));
        }
    }
}

impl TryFrom<u32> for common::Orientation {
    type Error = parse::errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (0..6).contains(&value) {
            Ok(Self { value: ScaledValue { value: value as u8, scale: 0u8..6 } })
        } else {
            return Err(parse::errors::Error::Format(parse::errors::Format::Data));
        }
    }
}