use clap::Parser;
use home::home_dir;
use std::fs;

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

    let dir = format!("{}/.rusty-pass-manager", home_dir().unwrap().display());
    fs::create_dir_all(&dir).unwrap();
    let gitignore = format!("{}/.gitignore", dir.clone());
    fs::write(gitignore, ".priv_key\n.pub_key\n.gitignore\n").unwrap();

    print!("{}[2J", 27 as char);
    // ask for pub and private key
    println!("Enter your public key:");
    let mut pub_key = String::new();
    std::io::stdin().read_line(&mut pub_key).unwrap();
    let pub_key = pub_key.trim();

    // clear the terminal
    print!("{}[2J", 27 as char);

    println!("Enter your private key:");
    let mut priv_key = String::new();
    std::io::stdin().read_line(&mut priv_key).unwrap();
    let priv_key = priv_key.trim();

    print!("{}[2J", 27 as char);

    let pub_key_file = format!("{}/.pub_key", dir.clone());
    fs::write(pub_key_file, pub_key).unwrap();

    let priv_key_file = format!("{}/.priv_key", dir.clone());
    fs::write(priv_key_file, priv_key).unwrap();

    // git init
    let git_init = format!("cd {} && git init", dir.clone());
    std::process::Command::new("sh")
        .arg("-c")
        .arg(git_init)
        .output()
        .expect("failed to execute process");

    // git add .gitignore
    let git_add = format!("cd {} && git add *", dir.clone());

    std::process::Command::new("sh")
        .arg("-c")
        .arg(git_add)
        .output()
        .expect("failed to execute process");

    // git commit -m "init"
    println!("starting commit");
    let git_commit = format!("cd {} && git commit -m \"init\"", dir.clone());

    std::process::Command::new("sh")
        .arg("-c")
        .arg(git_commit)
        .output()
        .expect("failed to execute process");

    println!("init");
}
