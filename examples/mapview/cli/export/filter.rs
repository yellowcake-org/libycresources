use clap::Parser;

#[derive(Parser)]
pub(crate) enum Filter {
    /// Optional filter for which layers to render
    Layers(Layers)
}

#[derive(Parser)]
pub(crate) struct Layers {
    #[clap(short, long)]
    pub(crate) floor: bool,
    #[clap(short, long)]
    pub(crate) overlay: bool,
    #[clap(short, long)]
    pub(crate) roof: bool,
    #[clap(short, long)]
    pub(crate) walls: bool,
    #[clap(short, long)]
    pub(crate) items: bool,
    #[clap(short, long)]
    pub(crate) misc: bool,
    #[clap(short, long)]
    pub(crate) scenery: bool,
    #[clap(short, long)]
    pub(crate) critters: bool,
}

impl Layers {
    pub(crate) fn all(&self) -> bool {
        let flags = [self.floor, self.overlay, self.roof, self.walls, self.items, self.misc, self.critters];
        flags.iter().min() == flags.iter().max()
    }
}

impl Default for Layers {
    fn default() -> Self {
        Self {
            floor: false,
            overlay: false,
            roof: false,
            walls: false,
            items: false,
            misc: false,
            scenery: false,
            critters: false,
        }
    }
}
