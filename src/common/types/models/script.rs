pub type Type = Kind<(), (), (), (), ()>;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Kind<Sys, Sp, T, I, C> {
    System(Sys),
    Spatial(Sp),
    Timed(T),
    Item(I),
    Critter(C),
}

impl TryFrom<u32> for Type {
    type Error = super::super::errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match (value >> u8::BITS * 3) as u8 {
            0 => Self::System(()),
            1 => Self::Spatial(()),
            2 => Self::Timed(()),
            3 => Self::Item(()),
            4 => Self::Critter(()),
            _ => return Err(Self::Error::Format)
        })
    }
}