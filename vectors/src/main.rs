fn main() {
    //empty vector must have a type
    let mut v: Vec<i32> = Vec::new();

    //otherwise vectors infer types at the time of assignment
    let mut v1 = vec!["john", "jack", "jacob"];

    v1.push("jill");
    v.push(2);

    println!("{:?}, {:?}", v, v1);

    //reading from a vector
    //using get and a simple reference
    let fetch = &v[0];
    let fetch1 = v1.get(2);

    println!("{:?}, {:?}", fetch, fetch1);

    //interating over vector values
    for i in &v1 {
        println!("\t{i}");
    }

    let mut v2 = vec![10, 20, 30];
    let v2 = add_ten(&mut v2).clone();
    println!("{:?}", v2);

    using_enums();
}

fn add_ten(vec: &mut Vec<i32>) -> &Vec<i32> {
    for i in &mut *vec {
        *i += 10;
    }
    vec
}

#[derive(Debug)]
enum Groups {
    Cities(String),
    NumericCodes(i32),
}

fn using_enums() {
    let japan = vec![
        Groups::Cities(String::from("Tokyo")),
        Groups::NumericCodes(1),
    ];

    let china = vec![
        Groups::Cities(String::from("Beijing")),
        Groups::NumericCodes(2),
    ];

    let places = vec![japan, china];

    println!("{:?}", places);
}
