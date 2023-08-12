use super::add;

pub fn edit(service_name: String, password: String) {
    add::add(service_name, password, Some(true));
}
