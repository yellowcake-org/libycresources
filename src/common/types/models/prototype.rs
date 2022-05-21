use crate::formats::pro;
use super::super::errors::Error;

impl TryFrom<u8> for pro::ObjectType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Error> {
        Ok(match value {
            0 => Self::Item(()),
            1 => Self::Critter(()),
            2 => Self::Scenery(()),
            3 => Self::Wall(()),
            4 => Self::Tile(()),
            5 => Self::Misc(()),
            _ => return Err(Error::Format)
        })
    }
}