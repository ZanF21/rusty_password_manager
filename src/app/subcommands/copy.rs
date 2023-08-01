use super::common;
use home::home_dir;

pub fn copy(service_name: String) {
    if !common::already_exists(service_name.clone()) {
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
