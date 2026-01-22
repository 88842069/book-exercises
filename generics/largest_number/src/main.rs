fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = largest_number(&number_list);
    println!("The largest number is {largest}");
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = largest_number(&number_list);
    println!("The largest number is {largest}");
    // let mut largest = &number_list[0];
    // for number in &number_list {
    // if number > largest {
    // largest = number;
    // }
    // }
}

fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
