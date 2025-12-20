//this probably can be made better
fn main() {
    let cardinals: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let lines: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four colly birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve fiddlers fiddling",
    ];

    for mut i in 1..13 {
        println!("The {} day of Christmas,", cardinals[i - 1]);
        println!("My true love sent to me");

        while i != 0 {
            while i != 0 {
                if i == 1 {
                    println!("{}.\n", lines[i - 1]);
                } else {
                    if i == 2 {
                        println!("{}, and", lines[i - 1]);
                    } else {
                        println!("{},", lines[i - 1]);
                    }
                }
                i -= 1;
            }
        }
    }
}
