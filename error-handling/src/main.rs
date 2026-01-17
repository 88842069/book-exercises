use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    match read_or_create_usernames_file() {
        Ok(_) => (),
        Err(e) => {
            println!("could not read or create file! {e:?}");
            return;
        }
    }

    let usernames = get_or_set_usernames().unwrap();
    println!("printing usernames:\n{usernames:?}");
}

fn get_or_set_usernames() -> Result<String, io::Error> {
    let usernames_file = File::open("usernames.txt");

    let mut usernames_file_copy = File::open("usernames.txt")?;
    let mut usernames = String::new();
    usernames_file_copy.read_to_string(&mut usernames)?;

    if !usernames.is_empty() {
        return Ok(usernames);
    } else {
        println!("file is empty! adding the admin user");
        fs::write("usernames.txt", "admin")?;
    }

    // here lies the clunky part
    File::open("usernames.txt")?;
    let mut usernames = String::new();
    usernames_file.unwrap().read_to_string(&mut usernames)?;
    Ok(usernames)
}

fn read_or_create_usernames_file() -> Result<u8, io::Error> {
    let file_exists = File::open("usernames.txt");
    match file_exists {
        Ok(_) => (),
        Err(e) => {
            println!("file does not exist!\n{e:?}\ncreating usernames.txt");
            File::create("usernames.txt")?;
        }
    }
    return Ok(0);
}
