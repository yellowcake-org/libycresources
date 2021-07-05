pub(crate) mod extract;
pub(crate) mod list;

use clap::Clap;

#[derive(Clap)]
#[clap(name = "undat", version)]
struct Options {
    #[clap(short, long)]
    input: String,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Clap)]
enum Action {
    #[clap()]
    List,
    #[clap()]
    Extract(Extract),
}

#[derive(Clap)]
struct Extract {
    output: String,
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

    match options.action {
        Action::List => {
            for entry in &entries {
                println!("{:}", &entry.path);
            }
        }
        Action::Extract(arguments) => {
            let result = extract::entry(&options.input, entries.as_slice(), &arguments.output);

            if let Err(error) = result {
                eprintln!("Error occured: {:?}", error);
            }
        }
    }
}
