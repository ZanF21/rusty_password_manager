use clap::Parser;
pub mod init;



/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Perform initialization tasks for Password Manager to run
    #[arg(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    init: bool
}

fn main() {
    let args = Args::parse();

    if args.init {
        init::init();
        return;
    }
}