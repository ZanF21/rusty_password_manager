use clap::Parser;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Perform initialization tasks for Password Manager to run
    #[arg(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    init: bool,

    /// Name of the person to greet
    #[arg(short, long, default_value = "World")]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value = "1")]
    count: u8,
}

fn main() {
    let args = Args::parse();

    if args.init {
        init();
        return;
    }

    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}

fn init() {
    // TODO: Implement init
    // 1. Create a directory in the user's home directory called .rusty-pass-manager
    // 2. passwords are stored in a file with the associated website/app name
    // 3. The file is encrypted with an ssh key
    // 4. the directory is also git initialized
    // if dir already exists, then ask twice if they want to overwrite and also ask for the ssh key
    // if the dir does not exist, then ask for the ssh key pub and private key and add them to .pub_key and .priv_key files
    // also create a .gitignore file and add the .priv_key and .pub_key and .gitignore files to it

    println!("init");
}
