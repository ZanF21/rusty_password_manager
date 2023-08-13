use chrono;
use home;
use magic_crypt::MagicCryptTrait;
use rpassword;
use serde_json;
use std::io::Write;

fn config_already_exists(path: String) -> bool {
    if std::path::Path::new(&path).exists() {
        return true;
    }
    false
}

fn ask_root_dir_location() -> String {
    let mut root_dir = String::new();
    print!(
        "PassMan Location (Default: '{}/.rusty'): ",
        home::home_dir().unwrap().display()
    );
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut root_dir)
        .expect("Couldn't read input");
    root_dir = root_dir.trim().to_string();
    if root_dir.is_empty() {
        root_dir = format!("{}/.rusty", home::home_dir().unwrap().display());
    }
    root_dir
}

fn ask_master_password() -> String {
    let mut master_password;
    loop {
        print!("Enter Master Password: ");
        std::io::stdout().flush().unwrap();
        master_password = rpassword::read_password().unwrap();
        print!("Re-Enter Master Password: ");
        std::io::stdout().flush().unwrap();
        let master_password_check = rpassword::read_password().unwrap();

        if master_password == master_password_check {
            break;
        } else {
            println!("Passwords do not match");
        }
    }
    master_password
}

#[allow(dead_code)]
pub fn update_last_verified() {
    let mut config = get_conf();
    config["last_verified"] = serde_json::Value::String(
        chrono::offset::Local::now()
            .format("%Y-%b-%d %H:%M:%S")
            .to_string(),
    );
    write_conf(config);
}

pub fn gen_conf() {
    let config_path = format!(
        "{}/.rusty/.config.json",
        home::home_dir().unwrap().display()
    );

    // Removing the config if thats what is required
    if config_already_exists(config_path.clone()) {
        println!("Config file already exists at {}", config_path);

        loop {
            print!("Do you want to overwrite it? (y/N): ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read input");
            if input.trim().to_lowercase() == "y" {
                break;
            } else if input.trim().to_lowercase() == "n" || input.trim() == "" {
                return;
            } else {
                println!("Invalid input");
            }
        }
    } else {
        std::fs::create_dir_all(format!("{}/.rusty", home::home_dir().unwrap().display())).unwrap();
    }

    let root_dir = ask_root_dir_location();
    let master_password = ask_master_password();
    let mut always_ask_password = false;
    loop {
        print!("Use Master Password to authenticate every single time? (y/N): ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input");
        if input.trim().to_lowercase() == "y" {
            always_ask_password = true;
            break;
        } else if input.trim().to_lowercase() == "n" || input.trim() == "" {
            break;
        }
    }
    let mcrypt = new_magic_crypt!(master_password.clone(), 256);
    let pass_phrase_hash = mcrypt.encrypt_str_to_base64("rizan did a great job making `rusty`");

    let created_time = chrono::offset::Local::now()
        .format("%Y-%b-%d %H:%M:%S")
        .to_string();

    let config = serde_json::json!({
        "master_password": master_password,
        "always_ask_password": always_ask_password,
        "date_created": created_time,
        "root_dir": root_dir,
        "master_pass_check": pass_phrase_hash,
        "last_verified": created_time
    });

    std::fs::write(config_path, config.to_string()).expect("Unable to write file");
}

pub fn get_conf() -> serde_json::Value {
    let config_path = format!(
        "{}/.rusty/.config.json",
        home::home_dir().unwrap().display()
    );
    let config = std::fs::read_to_string(config_path).expect("Unable to read file");
    let config: serde_json::Value = serde_json::from_str(&config).unwrap();
    config
}

pub fn write_conf(config: serde_json::Value) {
    let config_path = format!(
        "{}/.rusty/.config.json",
        home::home_dir().unwrap().display()
    );
    std::fs::write(config_path, config.to_string()).expect("Unable to write file");
}
