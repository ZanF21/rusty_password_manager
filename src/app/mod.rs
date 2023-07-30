use clap::Parser;
mod subcommands;
use subcommands::{add, copy, init, Subcommands};

use self::subcommands::show_all;

#[derive(Parser, Debug)]
#[command(
    author = "Rizan",
    version,
    about = "A Rusty but Trusty Password Manager",
    long_about = "Don't got time for this sadly .... move along ... come here next time :)"
)]
struct Rusty {
    #[clap(subcommand)]
    pub subcmd: subcommands::Subcommands,
}

pub fn run() {
    let application = Rusty::parse();

    match application.subcmd {
        Subcommands::Init => {
            println!("Initializing Password Manager");
            init::init();
        }

        Subcommands::Add {
            service_name,
            password,
        } => add::add(service_name, password),

        Subcommands::Copy { service_name } => {
            copy::copy(service_name);
        }
        Subcommands::ShowAll => {
            show_all::show_all();
        }
    }
}
