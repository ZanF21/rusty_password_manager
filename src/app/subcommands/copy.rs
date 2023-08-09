use magic_crypt::MagicCryptTrait;
use serde_json;
use std::io::Read;
use std::process::{Command, Stdio};

pub fn copy(service_name: String, config: serde_json::Value) {
    let root_dir = config["root_dir"].as_str().unwrap();
    let master_password = config["master_password"].as_str().unwrap();

    let mut file = std::fs::File::open(format!("{}/{}/password", root_dir, service_name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mcrypt = new_magic_crypt!(master_password, 256);
    let decrypted_string = mcrypt.decrypt_base64_to_string(&contents).unwrap();

    let echo = match Command::new("echo")
        .arg("-n")
        .arg(decrypted_string)
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn echo: {}", why),
        Ok(process) => process,
    };

    Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::from(echo.stdout.unwrap()))
        .spawn()
        .expect("Failed to execute command");

    println!("Password copied to clipboard");
}
