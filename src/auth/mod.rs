use super::app::conf;

#[allow(dead_code)]
fn verify_master_password() -> (bool, String) {
    // Check last seen time
    // - If greater than 5 minutes, ask for password and reset password in config (in mem only)
    // - If less than 5 minutes, return true and also have it stored in the config
    // - will write the password to config so that we cab use that in a later time
    let master_pass = "".to_string();
    (false, master_pass)
}

pub fn auth_user() -> (bool, String) {
    let config = conf::get_conf();
    if config["always_ask_password"].as_bool().unwrap() {
        // this does not work yet, will need to return master pass
        verify_master_password()
    } else {
        (
            true,
            config["master_password"].as_str().unwrap().to_string(),
        )
    }
}
