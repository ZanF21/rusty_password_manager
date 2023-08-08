use clipboard::{ClipboardContext, ClipboardProvider};
use magic_crypt::MagicCryptTrait;
use serde_json;
use std::io::Read;

pub fn copy(service_name: String, config: serde_json::Value) {
    let root_dir = config["root_dir"].as_str().unwrap();
    let master_password = config["master_password"].as_str().unwrap();

    let mut file = std::fs::File::open(format!("{}/{}", root_dir, service_name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mcrypt = new_magic_crypt!(master_password, 256);
    let decrypted_string = mcrypt.decrypt_base64_to_string(&contents).unwrap();

    // copy to clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(decrypted_string).unwrap();
    println!("Password copied to clipboard");
}
