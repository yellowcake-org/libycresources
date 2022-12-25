use clap::ArgEnum;

#[derive(ArgEnum, Clone)]
pub(crate) enum Darkness { None, Night, Dusk, Day }
