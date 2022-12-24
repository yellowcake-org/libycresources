use crate::common::traits::TryFromOptional;
use crate::common::types::geometry;
use crate::formats::pro::object::common::critter;
use crate::formats::pro::object::item::weapon;

#[derive(Debug, Eq, PartialEq)]
pub enum Kind {
    Item,
    Critter(Option<geometry::Orientation>, critter::Animation, Option<weapon::Animation>),
    Scenery,
    Wall,
    Tile,
    Misc,
    Background,
    Interface,
    Inventory,
    Head,
    Skilldex,
}

impl TryFrom<u32> for Kind {
    type Error = super::super::errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match (value >> u8::BITS * 3) as u8 {
            0 => Self::Item,
            1 => {
                Self::Critter(
                    match value >> 28 as u8 & 0b0111 {
                        0 => None,
                        value => Some(geometry::Orientation::try_from(
                            u32::try_from(value - 1).map_err(|_| { Self::Error::Format })?
                        )?)
                    },
                    critter::Animation::try_from((value >> 16) as u8)?,
                    weapon::Animation::try_from_optional(
                        u32::try_from((value >> 12) as u8 & 0b1111)
                            .map_err(|_| { Self::Error::Format })?,
                        0x00,
                    )?,
                )
            }
            2 => Self::Scenery,
            3 => Self::Wall,
            4 => Self::Tile,
            5 => Self::Misc,
            6 => Self::Interface,
            7 => Self::Inventory,
            8 => Self::Head,
            9 => Self::Background,
            10 => Self::Skilldex,
            _ => return Err(Self::Error::Format)
        })
    }
}