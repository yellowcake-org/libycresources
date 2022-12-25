use super::errors::Error;

pub mod sprite;
pub mod script;
pub mod prototype;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Identifier<Kind> {
    pub index: u16,
    pub kind: Kind,
}

impl<Kind> TryFrom<u32> for Identifier<Kind> where Kind: TryFrom<u32> {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            index: (value & 0b1111_1111_1111) as u16,
            kind: Kind::try_from(value).map_err(|_| Error::Format)?,
        })
    }
}