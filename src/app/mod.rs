use clap::Parser;
mod subcommands;
use subcommands::{add, init, copy, Subcommands};

#[derive(Parser, Debug)]
#[command(
    author = "Rizan",
    version,
    about = "A Rusty but Trusty Password Manager",
    long_about = "Don't got time for this sadly .... move along ... come here next time :)"
)]
struct Rusty {
    /// View all stored Passwords
    #[arg(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub view: bool,
    
    
    #[clap(subcommand)]
    pub subcmd: subcommands::Subcommands,
}

pub fn run() {
    let application = Rusty::parse();

    if application.view {
        // cd to ~/.rusty_password_manager and then tree . -d
        println!("Viewing all stored Passwords");
        std::process::Command::new("tree")
            .arg(".")
            .arg("-d")
            .current_dir(std::env::var("HOME").unwrap() + "/.rusty_password_manager")
            .spawn()
            .expect("Failed to execute command");
        return;
    }
    match application.subcmd {
        Subcommands::Init => {
            println!("Initializing Password Manager");
            init::init();
        }

        Subcommands::Add {
            service_name,
            password,
        } => add::add(service_name, password),

        Subcommands::Copy {
            service_name,
        } => {
            copy::copy(service_name);
        }
        
    }
}
