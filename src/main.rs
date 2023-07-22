pub mod app;
use crate::app::{subcommands, Rusty};
use clap::Parser;
use subcommands::{add, init, show, Subcommands};

fn main() {
    // let args = Args::parse();
    let application = Rusty::parse();
    // let is_verbose = app.verbose;

    match application.subcmd {
        Subcommands::Init => {
            println!("Initializing Password Manager");
            init::init();
        }
        Subcommands::Add {
            service_name,
            password,
        } => {
            // println!("Adding {} with password {}", service_name, password);
            add::add(service_name, password)
        }

        Subcommands::Show { service_name } => {
            show::show(service_name);
        }
    }
}
