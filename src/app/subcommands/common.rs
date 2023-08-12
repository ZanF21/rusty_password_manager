use std::io::Write;
use std::path::Path;

/// Checks if password already exists
pub fn already_exists(path: String) -> bool {
    if Path::new(&path).exists() {
        return true;
    }
    false
}

/// Ask for password (visible on the tty)
pub fn ask_password_print() -> String {
    print!("Enter password: ");
    std::io::stdout().flush().unwrap();
    let mut password = String::new();
    std::io::stdin()
        .read_line(&mut password)
        .expect("Couldn't read input");
    password.trim().to_string()
}
