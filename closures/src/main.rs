// giveaway tshirts to users
// if someone has a favorite colour they get that
// else they get whatever colour we currently have the most of

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

// #[derive(Debug)]
// struct User {
//     shirt_preference: Option<ShirtColor>,
// }

impl Inventory {
    fn giveaway(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
        user_pref.unwrap_or_else(|| self.current_max())
    }

    fn current_max(&self) -> ShirtColor {
        let mut red = 0;
        let mut blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => red += 1,
                ShirtColor::Blue => blue += 1,
            }
        }
        if red > blue {
            return ShirtColor::Red;
        } else {
            return ShirtColor::Blue;
        }
    }
}

fn main() {
    let current_inventory = Inventory {
        shirts: vec![
            ShirtColor::Red,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Blue,
        ],
    };

    println!("DEBUG: inventory: {:?}", current_inventory);

    let user_red = Some(ShirtColor::Red);
    let giveaway1 = current_inventory.giveaway(user_red);
    println!("DEBUG: giveaway {:?} to {:?}", giveaway1, user_red);

    let user_none = None;
    let giveaway2 = current_inventory.giveaway(user_none);
    println!("DEBUG: giveaway {:?} to {:?}", giveaway2, user_none);
}
