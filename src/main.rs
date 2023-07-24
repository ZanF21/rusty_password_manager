mod app;
// use home::home_dir;
// use std::io::Write;

fn main() {
    // take inpur 
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // let input = input.trim().to_string();
    // let service = input.split("/").last().unwrap();
    // let input = input.replace(format!("/{}", service).as_str(), "");
    // let mut file_path = format!("{}/.rusty-pass-manager/{}", home_dir().unwrap().display(), input.clone());

    // // remove last from input
    // println!("{}", file_path);

    // println!("{}", service);
    // println!("{}", input);
    // // if service and input are same then input is service
    // if input != service {
    //     println!("input is not service");
    //     std::fs::create_dir_all(file_path.clone()).expect("Couldn't create directory");
    //     file_path = format!("{}/{}", file_path, service);
    // } 

    // let mut file = std::fs::File::create(file_path).expect("Couldn't create file");
    // let mut password = String::new();
    // std::io::stdin().read_line(&mut password).unwrap();
    // password = password.trim().to_string();
    
    // file.write_all(password.as_bytes()).expect("Couldn't write to file");
    // println!("Done");

    app::run();
}
