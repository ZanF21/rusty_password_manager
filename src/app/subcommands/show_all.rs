use home::home_dir;

pub fn show_all() {
    let dir_path = format!("{}/.rusty-pass-manager", home_dir().unwrap().display());
    println!("Viewing all stored Passwords");
    std::process::Command::new("tree")
        .arg(".")
        .arg("-d")
        .arg("--nolinks")
        .arg("--noreport")
        .arg("-C")
        .current_dir(dir_path)
        .spawn()
        .expect("Failed to execute command");
}
