use clap::Parser;
mod subcommands;
use subcommands::{add, copy, create, show_all, Subcommands};

use crate::auth;
pub mod conf;
pub mod encode;

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
            conf::gen_conf();
        }

        Subcommands::Add {
            service_name,
            password,
        } => {
            let enc_pass = encode::encrypt(password);
            add::add(service_name, enc_pass);
        }
        Subcommands::Copy { service_name } => {
            let (auth_done, _) = auth::auth_user();
            if auth_done {
                copy::copy(service_name, conf::get_conf());
            } else {
                println!("Authentication failed");
            }
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
            let gen_pass = create::create(
                no_lowercase,
                no_uppercase,
                no_numbers,
                no_symbols,
                length,
                exclude_similar,
            );
            let enc_pass = encode::encrypt(gen_pass);
            add::add(service_name.clone(), enc_pass);
            copy::copy(service_name, conf::get_conf());
        }
    }
}
