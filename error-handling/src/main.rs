use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    // read file or create it if it doesnt exist
    match read_or_create_usernames_file() {
        Ok(_) => (),
        Err(e) => {
            println!("could not read or create file! {e:?}");
            return;
        }
    }

    // print users
    let usernames = get_or_set_usernames();

    println!("printing usernames:\n{usernames:?}");
}

fn get_or_set_usernames() -> String {
    let usernames_file = File::open("usernames.txt");
    let mut usernames = String::new();

    match usernames_file.unwrap().read_to_string(&mut usernames) {
        Ok(_) => (),
        Err(e) => return format!("could not read from file!\n{e:?}").to_string(),
    }

    if !usernames.is_empty() {
        return usernames;
    } else {
        println!("file is empty! adding the admin user");
        let add_first_user = fs::write("usernames.txt", "admin");
        match add_first_user {
            Ok(_) => {
                println!("added admin");
            }

            Err(e) => {
                println!(
                    "could not add the admin user
                    error: {e:?}"
                );
                return format!(
                    "file is empty and we could not add the admin user
                    {e:?}"
                )
                .to_string();
            }
        }
    }

    let usernames_file = File::open("usernames.txt");
    let mut usernames = String::new();

    match usernames_file.unwrap().read_to_string(&mut usernames) {
        Ok(_) => (),
        Err(e) => return format!("could not read from file after adding admin\n{e:?}").to_string(),
    }
    usernames
}

fn read_or_create_usernames_file() -> Result<File, io::Error> {
    let file_exists = File::open("usernames.txt");

    match file_exists {
        Ok(filename) => return Ok(filename),
        Err(e) => {
            println!("file does not exist!\n{e:?}\ncreating usernames.txt");

            let create_file = File::create("usernames.txt");

            match create_file {
                Ok(filename) => {
                    println!("created file: {filename:?}");
                    return Ok(filename);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}
