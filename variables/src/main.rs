// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main() {
//     // let tup: (i32, f64) = (500, 2.91);
//     let tup = (500, 2.91);
//     let t: bool = true;
//     let c: char = '✅';
//
//     println!("t is {t}\nc is {c}\nand tup is {:?}\n\n\n", tup);
//
//     // let (x: i32, y: f64, z) = tup;
//     let (x, y) = tup;
//     println!("the second value is: {}", y);
//     println!("the second value is: {}", tup.1);
// }

fn main() {
    let months = ["jan", "feb", "mar", "apr", "may", "jun"];
    let ratings: [i32; 3] = [1, 2, 3];
    let mut employees = ["do kwon", "sbf", "tabasco", "kyle"];

    println!("employees are: {:?}", employees);

    let mut employee_of_the_month: (&str, &str, i32) = ("jan", "do kwon", 2);

    let (x, y, z) = employee_of_the_month;
    println!(
        "\nin {x}, {y} was the employee of the month with a rating of {}",
        employee_of_the_month.2
    );

    println!("\ncaroline replaced kyle");

    employees[3] = "caroline";

    println!("employees are: {:?}", employees);

    employee_of_the_month = (months[1], employees[1], ratings[2]);
    let (x, y, z) = employee_of_the_month;
    println!("\nin {x}, {y} was the employee of the month with a rating of {z}");

    employee_of_the_month = (months[2], employees[2], ratings[1]);
    let (x, y, z) = employee_of_the_month;
    println!("\nin {x}, {y} was the employee of the month with a rating of {z}");

    employee_of_the_month = (months[3], employees[3], ratings[0]);
    let (x, y, z) = employee_of_the_month;
    println!("\nin {x}, {y} was the employee of the month with a rating of {z}");
}
