use super::errors::Error;

pub mod sprite;
pub mod script;
pub mod prototype;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Identifier<Kind> {
    pub raw: u32,
    pub index: u16,

    pub kind: Kind,
}

impl<Kind> TryFrom<u32> for Identifier<Kind> where Kind: TryFrom<u8> {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            raw: value,
            index: (value & 0b1111_1111_1111) as u16,
            kind: Kind::try_from((value >> u8::BITS * 3) as u8).map_err(|_| Error::Format)?,
        })
    }
}