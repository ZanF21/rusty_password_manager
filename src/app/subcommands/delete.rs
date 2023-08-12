use super::super::conf;

pub fn delete(service_name: String) {
    let conf = conf::get_conf();
    let root_dir = conf["root_dir"].as_str().unwrap();
    let file_path = format!("{}/{}/password", root_dir, service_name);
    // delete the fiel
    std::fs::remove_file(file_path).expect("Unable to delete file");
    // find . -type d -empty -delete
    std::process::Command::new("find")
        .arg(".")
        .arg("-type")
        .arg("d")
        .arg("-empty")
        .arg("-delete")
        .current_dir(root_dir)
        .spawn()
        .expect("Failed to execute command");
    println!("Password for {} deleted", service_name);
}
