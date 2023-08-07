use chrono;
use home;
use rpassword;
use serde_json;
use std::io::Write;

fn config_already_exists(path: String) -> bool {
    if std::path::Path::new(&path).exists() {
        return true;
    }
    false
}

#[allow(dead_code)]
pub fn auth_user(master_password: String) -> bool {
    print!("Confirm Master Password: {}", master_password);
    std::io::stdout().flush().unwrap();
    true
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

    print!("Use Master Password to authenticate every single time? (y/N): ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input");
    let master_pass_auth = input.trim().to_lowercase() == "y";
    // println!("Noet: The master password is stored in the config file anyways ..... \n");

    let config = serde_json::json!({
        "master_password": master_password,
        "master_pass_auth": master_pass_auth,
        "date_created": chrono::offset::Local::now().format("%Y-%b-%d %H:%M:%S").to_string(),
        "root_dir": root_dir,
        "master_pass_check": "TODO",
        "last_used": "TODO"
    });

    std::fs::write(config_path, config.to_string()).expect("Unable to write file");
}
