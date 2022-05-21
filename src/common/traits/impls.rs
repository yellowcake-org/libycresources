use std::ops::Range;

use errors::Error;

use crate::common::types::errors;
use crate::common::types::geometry::{Coordinate, Orientation, Scaled};
use crate::common::types::space::Elevation;

impl TryFrom<i32> for Coordinate<u8, Range<u8>> {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from(u32::try_from(value).map_err(|_| Error::Format)?)
    }
}

impl TryFrom<u32> for Coordinate<u8, Range<u8>> {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        const SCALE: Range<u32> = u32::MIN..200;

        if (u32::MIN..SCALE.end.pow(2)).contains(&value) {
            let x = value / SCALE.end;
            let y = value - (x * SCALE.end);

            let x = u8::try_from(x).map_err(|_| Error::Format)?;
            let y = u8::try_from(y).map_err(|_| Error::Format)?;

            Ok(Self {
                x: Scaled { value: x, scale: u8::MIN..(SCALE.end as u8) },
                y: Scaled { value: y, scale: u8::MIN..(SCALE.end as u8) },
            })
        } else {
            return Err(Error::Format);
        }
    }
}

impl TryFrom<u32> for Elevation {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        const SCALE: Range<u8> = u8::MIN..3;
        let value = u8::try_from(value).map_err(|_| Error::Format)?;

        if SCALE.contains(&value) {
            Ok(Self { level: Scaled { value, scale: SCALE } })
        } else {
            return Err(Error::Format);
        }
    }
}

impl TryFrom<u32> for Orientation {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        const SCALE: Range<u8> = u8::MIN..6;
        let value = u8::try_from(value).map_err(|_| Error::Format)?;

        if SCALE.contains(&value) {
            Ok(Self { value: Scaled { value, scale: SCALE } })
        } else {
            return Err(Error::Format);
        }
    }
}