// Building up to structs
//
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels",
//         area(width1, height1)
//     );
// }
//
// fn area(w: u32, h: u32) -> u32 {
//     w * h
// }

// fn main() {
//     let rect1 = (30, 50);
//
//     println!("The area of the rectangle is {} square pixels", area(rect1))
// }
//
// fn area(dims: (u32, u32)) -> u32 {
//     dims.0 * dims.1
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

fn main() {
    let scale = 3;
    let my_rec = Rectangle {
        width: dbg!(30 * scale),
        height: 40,
    };

    dbg!(&my_rec);

    // println!(
    //     "The area of the rectangle is {} square pixels",
    //     area(&my_rec)
    // );
    //
    // println!("rectangle is {my_rec:?}");
    // println!("--------");
    // println!("rectangle is {my_rec:#?}");
}
