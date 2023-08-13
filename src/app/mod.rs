use clap::Parser;
mod subcommands;
use subcommands::{add, common, copy, create, delete, edit, show_all, Subcommands};

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

        Subcommands::Add { service_name } => {
            if !common::already_exists(format!(
                "{}/.rusty/.config.json",
                std::env::var("HOME").unwrap()
            )) {
                println!("Init Not Done!!\nTry running `rusty init`");
                return;
            }
            let password = common::ask_password_print();
            let enc_pass = encode::encrypt(password);
            add::add(service_name, enc_pass, None);
        }
        Subcommands::Copy { service_name } => {
            if !common::already_exists(format!(
                "{}/.rusty/.config.json",
                std::env::var("HOME").unwrap()
            )) {
                println!("Init Not Done!!\nTry running `rusty init`");
                return;
            }
            auth::auth_user();
            copy::copy(service_name, conf::get_conf());
        }
        Subcommands::ShowAll => {
            if !common::already_exists(format!(
                "{}/.rusty/.config.json",
                std::env::var("HOME").unwrap()
            )) {
                println!("Init Not Done!!\nTry running `rusty init`");
                return;
            }
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
            if !common::already_exists(format!(
                "{}/.rusty/.config.json",
                std::env::var("HOME").unwrap()
            )) {
                println!("Init Not Done!!\nTry running `rusty init`");
                return;
            }
            let enc_pass = encode::encrypt(gen_pass);
            add::add(service_name.clone(), enc_pass, None);
            copy::copy(service_name, conf::get_conf());
        }
        Subcommands::Edit { service_name } => {
            auth::auth_user();
            let password = common::ask_password_print();
            let enc_pass = encode::encrypt(password);
            edit::edit(service_name, enc_pass);
        }
        Subcommands::Delete { service_name } => {
            auth::auth_user();
            delete::delete(service_name);
        }
    }
}

#[test]
fn verify_rusty() {
    use clap::CommandFactory;
    Rusty::command().debug_assert()
}
