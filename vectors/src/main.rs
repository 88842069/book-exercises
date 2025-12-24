use std::io;

fn main() {
    let mut friends: Vec<String> = Vec::new();

    friends = init_list();
    println!("you're friends with {:?}", friends);
    println!("1. update\n2. quit");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("failed to read line!");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            todo!();
        }
    };

    if choice == 1 {
        update_list(&mut friends);
        println!("updated friends list {:?}", friends);
    } else {
        return;
    }
}

fn update_list(friends: &mut Vec<String>) {
    println!("\t...your friends as of now are: {:?}", friends);
    println!("...which one would you like to update?");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line!");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            todo!();
        }
    };

    println!("...and you would like to replace them with: ");
    let mut to_update_with = String::new();

    io::stdin()
        .read_line(&mut to_update_with)
        .expect("failed to read line!");

    friends[index - 1] = to_update_with;
}

fn init_list() -> Vec<String> {
    println!("how many friends do you have?");

    let mut friend_count = String::new();

    io::stdin()
        .read_line(&mut friend_count)
        .expect("failed to read line!");

    let friend_count: u32 = match friend_count.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            todo!();
        }
    };

    let mut names = Vec::new();

    for i in 0..friend_count {
        println!("\tenter name of friend {}", i + 1);

        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("failed to read line!");

        names.push(name);
    }

    names
}
