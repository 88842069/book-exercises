use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    let username = get_username_or_create_file().unwrap();
    if username == "" {
        println!("empty!");
    } else {
        println!("{:?}", username);
    }
}

fn get_username_or_create_file() -> Result<String, io::Error> {
    let check_username_file = File::open("username.txt");

    let mut username_file = match check_username_file {
        Ok(filename) => filename,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => match File::create("username.txt") {
                Ok(file_created) => {
                    println!("created file, now writing...");
                    fs::write("{file_created}", "admin");
                    file_created
                }
                Err(error_creating_file) => {
                    panic!("error creating file! {error_creating_file:?}")
                }
            },
            _ => panic!("cant open file!"),
        },
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
