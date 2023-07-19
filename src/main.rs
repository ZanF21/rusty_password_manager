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
    let dir = format!("{}/.rusty-pass-manager", home_dir().unwrap().display());

    // Check if directory exists
    if fs::metadata(&dir).is_ok() {
        println!("Directory already exists: {}", dir);
        println!("Do you want to overwrite it? (y/N)");
        let mut overwrite = String::new();
        std::io::stdin().read_line(&mut overwrite).unwrap();
        let overwrite = overwrite.trim();
        if overwrite == "y" {
            fs::remove_dir_all(&dir).unwrap();
        } else {
            println!("Exiting...");
            return;
        }
    }
    fs::create_dir_all(&dir).unwrap();
    let gitignore = format!("{}/.gitignore", dir.clone());
    fs::write(gitignore, ".priv_key\n.pub_key\n.gitignore\n").unwrap();

    // Setup Encryption Keys
    loop {
        print!("{}[2J", 27 as char);
        println!("Setting up Encryption Keys");
        println!("--------------------------");
        println!("1. Generate new ssh key (not implemented)");
        println!("2. Use existing ssh key");
        println!("3. Enter ssh key manually");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };
        match choice {
            1 => {
                println!("1 but not implemented");
                break;
            }
            2 => {
                println!("2 but not implimented");
                break;
            }
            3 => {
                println!("3 but not implimented");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }

    let git_init = format!("cd {} && git init", dir.clone());
    std::process::Command::new("sh")
        .arg("-c")
        .arg(git_init)
        .output()
        .expect("failed to execute process");

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

    println!("created directory: {}", dir);
    println!();
}
