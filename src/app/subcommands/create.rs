use super::add;
use super::copy;
use passwords;

pub fn create(
    service_name: String,
    no_lowercase: bool,
    no_uppercase: bool,
    no_numbers: bool,
    no_symbols: bool,
    length: usize,
    exclude_similar: bool,
) {
    let password = passwords::PasswordGenerator::new()
        .exclude_similar_characters(exclude_similar)
        .length(length)
        .symbols(!no_symbols)
        .numbers(!no_numbers)
        .lowercase_letters(!no_lowercase)
        .uppercase_letters(!no_uppercase)
        .generate_one()
        .unwrap();

    add::add(service_name.clone(), password);
    copy::copy(service_name);
}
