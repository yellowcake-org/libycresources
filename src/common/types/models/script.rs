#[derive(Debug)]
pub enum Kind {
    Spatial,
    Item,
    Scenery,
    Critter,
}

impl TryFrom<u8> for Kind {
    type Error = super::super::errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Spatial,
            2 => Self::Item,
            3 => Self::Scenery,
            4 => Self::Critter,
            _ => return Err(Self::Error::Format)
        })
    }
}