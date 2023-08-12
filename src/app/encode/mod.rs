use super::conf;
use magic_crypt::MagicCryptTrait;
use walkdir::WalkDir;

pub fn encrypt(password: String) -> String {
    let conf = conf::get_conf();
    let master_password = conf["master_password"].as_str().unwrap();

    let mcrypt = new_magic_crypt!(master_password, 256);

    mcrypt.encrypt_str_to_base64(password)
}

#[allow(dead_code)]
pub fn flip_all_pass(encode: u8) {
    let conf = conf::get_conf();
    let master_password = conf["master_password"].as_str().unwrap();
    let root_dir = conf["root_dir"].as_str().unwrap();

    for file in WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            let mcrypt = new_magic_crypt!(master_password, 256);
            let pass_path = file.path().to_str().unwrap().to_string();
            let pass = std::fs::read_to_string(pass_path.clone()).expect("Unable to read file");

            if encode == 0 {
                let dec_pass = mcrypt.decrypt_base64_to_string(pass).unwrap();
                std::fs::write(pass_path, dec_pass).expect("Unable to write file");
            } else {
                let enc_pass = mcrypt.encrypt_str_to_base64(pass);
                std::fs::write(pass_path, enc_pass).expect("Unable to write file");
            }
        }
    }
}
