use clap::ArgEnum;

#[derive(ArgEnum)]
pub(crate) enum Darkness { None, Night, Dusk, Day }

impl Clone for Darkness {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Night => Self::Night,
            Self::Dusk => Self::Dusk,
            Self::Day => Self::Day,
        }
    }
}
