use home::home_dir;
use std::path::Path;

/// Checks if password already exists
pub fn already_exists(path: String) -> bool {
    let file_path = format!("{}/.rusty/{}", home_dir().unwrap().display(), path);
    if Path::new(&file_path).exists() {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_already_exists() {
        assert!(!already_exists("test".to_string()));
    }
}
