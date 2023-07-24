use home::home_dir;
use std::io::Write;
use std::path::Path;

/// Checks if password already exists
fn already_exists(service_name: String) -> bool {
    let file_path = format!(
        "{}/.rusty-pass-manager/{}",
        home_dir().unwrap().display(),
        service_name
    );
    if Path::new(&file_path).exists() {
        return true;
    }
    false
}

/// ### Add a new password
/// - Add a new password to the password manager
pub fn add(service_name: String, password: String) {
    if already_exists(service_name.clone()) {
        println!("Password for {} already exists", service_name.clone());
        // maybe ask for edit
        return;
    }
    let file_path = format!(
        "{}/.rusty-pass-manager/{}",
        home_dir().unwrap().display(),
        service_name.clone()
    );
    let pass_path = format!("{}/{}", file_path, "password");

    std::fs::create_dir_all(file_path.clone()).expect("Couldn't create directory");
    let mut file = std::fs::File::create(pass_path).expect("Couldn't create file");
    file.write_all(password.as_bytes())
        .expect("Couldn't write to file");
    println!("Password for {} added", service_name);
}
