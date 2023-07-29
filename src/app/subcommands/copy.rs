use home::home_dir;
use std::path::Path;

fn already_exists(service_name: &String) -> bool {
    let file_path = format!(
        "{}/.rusty-pass-manager/{}/password",
        home_dir().unwrap().display(),
        service_name
    );
    if Path::new(&file_path).exists() {
        return true;
    }
    false
}

pub fn copy(service_name: String) {
    if !already_exists(&service_name) {
        // print the pass on screen and copy to clip board
        println!("Password for {} does not exist", service_name);
    } else {
        std::process::Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .arg(format!(
                "{}/.rusty-pass-manager/{}/password",
                home_dir().unwrap().display(),
                service_name
            ))
            .spawn()
            .expect("Couldn't copy password");
        println!("Password for {} copied to clipboard", service_name)
    }
}
