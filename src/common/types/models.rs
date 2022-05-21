use super::errors::Error;

pub mod sprite;
pub mod script;
pub mod prototype;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Identifier<Kind> {
    pub kind: Kind,
    pub value: u16,
}

impl<Kind> TryFrom<u32> for Identifier<Kind> where Kind: TryFrom<u8> {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: Kind::try_from((value >> u8::BITS * 3) as u8).map_err(|_| Error::Format)?,
            value: (value & 0xFFFFFFFF) as u16,
        })
    }
}