use super::conf;
use magic_crypt::MagicCryptTrait;

pub fn encrypt(password: String) -> String {
    let conf = conf::get_conf();
    let master_password = conf["master_password"].as_str().unwrap();

    let mcrypt = new_magic_crypt!(master_password, 256);

    mcrypt.encrypt_str_to_base64(password)
}
