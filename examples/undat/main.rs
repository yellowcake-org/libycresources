pub(crate) mod extract;
pub(crate) mod list;

use clap::Clap;

#[derive(Clap)]
#[clap(name = "undat", version)]
struct Options {
    input: String,
    #[clap(short, long)]
    list: bool,
    #[clap(short, long)]
    extract: Option<String>,
}

fn main() {
    let options = Options::parse();

    let entries = match list::entries(&options.input) {
        Err(error) => {
            eprintln!("Error occured: {:?}", error);
            return;
        }
        Ok(value) => value,
    };

    if options.list {
        for entry in &entries {
            println!("{:}", &entry.path);
        }
    } else if let Some(output) = &options.extract {
        if let Err(error) = extract::entry(&options.input, entries.as_slice(), &output) {
            eprintln!("Error occured: {:?}", error);
        }
    } else {
        eprintln!("No command passed.");
    }
}
