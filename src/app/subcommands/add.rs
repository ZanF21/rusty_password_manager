use super::{super::conf, common};
use std::io::Write;

/// ### Add a new password
/// - Add a new password to the password manager
pub fn add(service_name: String, password: String, force_edit: Option<bool>) {
    let conf = conf::get_conf();
    let root_dir = conf["root_dir"].as_str().unwrap();
    let mut update = false;

    let force_edit = force_edit.unwrap_or(false);
    if force_edit {
        if !common::already_exists(format!("{}/{}/password", root_dir, service_name.clone())) {
            println!("Password for {} does not exist", service_name);
            println!("Consider adding the password for {}", service_name);
            println!("Use `rusty add {}`", service_name);
            return;
        }
        update = true;
    } else if common::already_exists(format!("{}/{}/password", root_dir, service_name.clone())) {
        println!("Password for {} already exists", service_name);
        loop {
            print!("Do you want to overwrite it? (y/N): ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read input");
            if input.trim().to_lowercase() == "y" {
                update = true;
                break;
            } else if input.trim().to_lowercase() == "n" || input.trim() == "" {
                return;
            }
        }
    }

    let file_path = format!("{}/{}", root_dir, service_name.clone());
    let pass_path = format!("{}/{}", file_path, "password");

    std::fs::create_dir_all(file_path).expect("Couldn't create directory");
    let mut file = std::fs::File::create(pass_path).expect("Couldn't create file");
    file.write_all(password.as_bytes())
        .expect("Couldn't write to file");
    if update {
        println!("Password for {} updated", service_name);
    } else {
        println!("Password for {} added", service_name);
    }
}
