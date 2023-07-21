use home::home_dir;
use serde_json;
use std::fs;
use std::io::{self, Write};

fn create_open_config() -> serde_json::Value {
    let config = format!(
        r#"{{
        "priv_key_path": "",
        "pub_key_path": ""
        }}"#
    );
    let config: serde_json::Value = serde_json::from_str(&config).unwrap();
    config
}

fn save_config(config: String) {
    let dir = format!("{}/.rusty-pass-manager", home_dir().unwrap().display());
    let config_path = format!("{}/config.json", dir.clone());
    fs::write(config_path, config).unwrap();
}

// add two key value pairs to config
fn add_save_config(config: &mut serde_json::Value, priv_key_path: String, pub_key_path: String) {
    config["priv_key_path"] = serde_json::Value::String(priv_key_path);
    config["pub_key_path"] = serde_json::Value::String(pub_key_path);
    let config = serde_json::to_string_pretty(&config).unwrap();
    save_config(config);
}

fn encryp_2() {
    let config = create_open_config();
    print!("Enter private key path   (default: ~/.ssh/id_ed25519): ");
    io::stdout().flush().expect("Couldn't flush stdout");
    let mut priv_key_path = String::new();
    std::io::stdin().read_line(&mut priv_key_path).unwrap();
    priv_key_path = priv_key_path.trim().to_string();
    if priv_key_path == "" {
        priv_key_path = format!("{}/.ssh/id_ed25519", home_dir().unwrap().display());
    }
    print!("Enter public key path    (default: ~/.ssh/id_ed25519.pub): ");
    io::stdout().flush().expect("Couldn't flush stdout");
    let mut pub_key_path = String::new();
    std::io::stdin().read_line(&mut pub_key_path).unwrap();
    pub_key_path = pub_key_path.trim().to_string();
    if pub_key_path == "" {
        pub_key_path = format!("{}/.ssh/id_ed25519.pub", home_dir().unwrap().display());
    }
    add_save_config(&mut config.clone(), priv_key_path, pub_key_path);
}

pub fn init() {
    let dir = format!("{}/.rusty-pass-manager", home_dir().unwrap().display());

    loop {
        if fs::metadata(&dir).is_ok() {
            println!("Directory already exists: {}", dir);
            print!("Do you want to overwrite it? (y/N): ");
            io::stdout().flush().expect("Couldn't flush stdout");
            let mut overwrite = String::new();
            std::io::stdin().read_line(&mut overwrite).unwrap();
            let overwrite = overwrite.trim();
            if overwrite == "y" || overwrite == "Y" {
                fs::remove_dir_all(&dir).unwrap();
                break;
            } else if overwrite == "N" || overwrite == "n" {
                println!("Exiting...");
                return;
            }
        }
    }
    fs::create_dir_all(&dir).unwrap();
    let gitignore_path = format!("{}/.gitignore", dir.clone());
    fs::write(gitignore_path, ".gitignore\n.config.json\n.completions/*").unwrap();

    let completions_path = format!("{}/.completions", dir.clone());
    fs::create_dir_all(&completions_path).unwrap();

    print!("{}[2J", 27 as char);
    // Setup Encryption Keys
    loop {
        println!("Setting up Encryption Keys");
        println!("--------------------------");
        println!("1. Generate new ssh key (not implemented)");
        println!("2. Use existing ssh key");
        println!("3. Enter ssh key manually (not implemented)");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("{}[2J", 27 as char);
                println!("Invalid choice");
                continue;
            }
        };
        match choice {
            1 => {
                println!("1 but not implemented");
                println!("Starting 2");
                encryp_2();
                break;
            }
            2 => {
                encryp_2();
                break;
            }
            3 => {
                println!("3 but not implimented");
                println!("Starting 2");
                encryp_2();
                break;
            }
            _ => {
                print!("{}[2J", 27 as char);
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

    println!("created directory: {}", dir);
    println!();
}
