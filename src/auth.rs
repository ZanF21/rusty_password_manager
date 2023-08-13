use super::app::conf;
use magic_crypt::MagicCryptTrait;
use std::io::Write;

fn verify_master_password(mut config: serde_json::Value) {
    let last_verified_time_str = config["last_verified"].as_str().unwrap();
    let curr_time_string = chrono::offset::Local::now()
        .format("%Y-%b-%d %H:%M:%S")
        .to_string();
    let last_verified_time =
        chrono::NaiveDateTime::parse_from_str(last_verified_time_str, "%Y-%b-%d %H:%M:%S");

    match last_verified_time {
        Ok(_) => {}
        Err(_) => {
            panic!("Last Verified Time is not in correct format")
        }
    }
    let last_verified_time = last_verified_time.unwrap();
    let curr_time =
        chrono::NaiveDateTime::parse_from_str(&curr_time_string, "%Y-%b-%d %H:%M:%S").unwrap();
    let time_diff = curr_time.signed_duration_since(last_verified_time);
    if time_diff.num_minutes() > 5 {
        print!("Enter Master Password: ");
        std::io::stdout().flush().unwrap();
        let password = rpassword::read_password().unwrap();
        let pass_phrase_hash = config["master_pass_check"].as_str().unwrap();
        let crypter = magic_crypt::new_magic_crypt!(password.clone(), 256);
        let decrypted = crypter.decrypt_base64_to_string(pass_phrase_hash);
        match decrypted {
            Ok(_) => {}
            Err(_) => {
                println!("ERROR: Wrong Master Password!!");
                std::process::exit(1);
            }
        }
        if decrypted.unwrap() == "rizan did a great job making `rusty`" {
            config["last_verified"] = serde_json::Value::String(curr_time_string);
            config["master_password"] = serde_json::Value::String(password);

            conf::write_conf(config.clone());
        } else {
            println!("Wrong Master Password!!");
            std::process::exit(1);
        }
    }
}

pub fn auth_user() {
    let config = conf::get_conf();
    if config["always_ask_password"].as_bool().unwrap() {
        verify_master_password(config);
    }
}
