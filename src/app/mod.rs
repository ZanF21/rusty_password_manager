use clap::Parser;
mod subcommands;
use passwords;
use subcommands::{add, copy, init, show_all, Subcommands};

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
        Subcommands::Create {
            service_name,
            no_lowercase,
            no_uppercase,
            no_numbers,
            no_symbols,
            length,
            exclude_similar,
        } => {
            let password = passwords::PasswordGenerator::new()
                .exclude_similar_characters(exclude_similar)
                .length(length)
                .symbols(!no_symbols)
                .numbers(!no_numbers)
                .lowercase_letters(!no_lowercase)
                .uppercase_letters(!no_uppercase)
                .generate_one()
                .unwrap();

            add::add(service_name.clone(), password);
            copy::copy(service_name);
        }
    }
}
