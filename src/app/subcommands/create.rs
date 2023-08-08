use passwords;

pub fn create(
    no_lowercase: bool,
    no_uppercase: bool,
    no_numbers: bool,
    no_symbols: bool,
    length: usize,
    exclude_similar: bool,
) -> String {
    let password = passwords::PasswordGenerator::new()
        .exclude_similar_characters(exclude_similar)
        .length(length)
        .symbols(!no_symbols)
        .numbers(!no_numbers)
        .lowercase_letters(!no_lowercase)
        .uppercase_letters(!no_uppercase)
        .generate_one()
        .unwrap();

    password
}
