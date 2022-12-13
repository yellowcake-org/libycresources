use clap::ArgEnum;

#[derive(ArgEnum, Clone)]
pub(crate) enum Elevation { First, Second, Third }
