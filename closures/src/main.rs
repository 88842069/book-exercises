// giveaway tshirts to users
// if someone has a favorite colour they get that
// else they get whatever colour we currently have the most of

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
enum ShirtColor {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct User {
    shirt_preference: Option<ShirtColor>,
}

impl Inventory {
    fn giveaway(current_inventory: &mut Self, user: &User) -> ShirtColor {
        let pref = user.shirt_preference;
        if pref == None {
            let current_max = Self::current_max(current_inventory);
            Self::update_inventory(current_inventory, current_max);
            return current_max;
        } else {
            Self::update_inventory(current_inventory, pref.unwrap());
            return pref.unwrap();
        }
    }

    fn update_inventory(current_inventory: &mut Self, current_max: ShirtColor) {
        let mut temp_inventory = current_inventory.shirts.clone();
        temp_inventory.sort();

        let mut i = 0;
        for color in temp_inventory {
            if color == current_max {
                current_inventory.shirts.swap_remove(i);
                return;
            }
            i += 1;
        }
    }

    fn current_max(current_inventory: &mut Self) -> ShirtColor {
        let mut green = 0;
        let mut blue = 0;
        let mut red = 0;

        let tmp_inventory = current_inventory.shirts.clone();
        for color in tmp_inventory {
            match color {
                ShirtColor::Red => red += 1,
                ShirtColor::Blue => blue += 1,
                ShirtColor::Green => green += 1,
            }
        }

        if blue > green && blue > red {
            return ShirtColor::Blue;
        };

        if green > blue && green > red {
            return ShirtColor::Green;
        };

        if red > blue && red > green {
            return ShirtColor::Red;
        };

        if blue == red && blue == green {
            return ShirtColor::Blue;
        };

        if blue == red && blue < green {
            return ShirtColor::Green;
        };

        if blue == red && blue > green {
            return ShirtColor::Blue;
        };

        if green == blue && green > red {
            return ShirtColor::Blue;
        };

        if green == blue && green < red {
            return ShirtColor::Red;
        };

        if red == green && green > blue {
            return ShirtColor::Green;
        };

        if red == green && green < blue {
            return ShirtColor::Blue;
        } else {
            return ShirtColor::Green;
        }
    }
}

fn main() {
    let mut current_inventory = Inventory {
        shirts: vec![
            ShirtColor::Red,
            ShirtColor::Red,
            ShirtColor::Green,
            ShirtColor::Green,
            ShirtColor::Green,
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Blue,
        ],
    };

    let user_red = User {
        shirt_preference: Some(ShirtColor::Red),
    };
    let user_green = User {
        shirt_preference: Some(ShirtColor::Green),
    };
    let user_none = User {
        shirt_preference: None,
    };

    println!("DEBUG: inventory: {:?}", current_inventory);
    println!("--------------------");

    println!(
        "DEBUG: giveaway {:?} to {:?}",
        Inventory::giveaway(&mut current_inventory, &user_red),
        user_red
    );

    println!("DEBUG: current inventory: {:?}", current_inventory);

    println!("--------------------");

    println!(
        "DEBUG: giveaway {:?} to {:?}",
        Inventory::giveaway(&mut current_inventory, &user_green),
        user_green
    );

    println!("DEBUG: current inventory: {:?}", current_inventory);
    println!("--------------------");

    println!(
        "DEBUG: giveaway {:?} to {:?}",
        Inventory::giveaway(&mut current_inventory, &user_none),
        user_none
    );

    println!("DEBUG: current inventory: {:?}", current_inventory);
    println!("--------------------");
}
