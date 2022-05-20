pub mod sprite;
pub mod script;
pub mod prototype;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Identifier<Kind> {
    pub kind: Kind,
    pub value: u16,
}

impl<Kind> TryFrom<u32> for Identifier<Kind> where Kind: TryFrom<u8> {
    type Error = super::errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: match Kind::try_from((value >> u8::BITS * 3) as u8) {
                Ok(value) => value,
                Err(_) => return Err(super::errors::Error::Format),
            },
            value: (value & 0xFFFFFFFF) as u16,
        })
    }
}