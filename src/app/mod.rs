use clap::Parser;
pub mod subcommands;

#[derive(Parser, Debug)]
#[command(
    author = "Rizan",
    version,
    about = "A Rusty but Trusty Password Manager",
    long_about = "Don't got time for this sadly .... move along ... come here next time :)"
)]
pub struct Rusty {
    /// Sets the level of verbosity
    #[clap(short, long)]
    pub verbose: bool,
    #[clap(subcommand)]
    pub subcmd: subcommands::Subcommands,
}
