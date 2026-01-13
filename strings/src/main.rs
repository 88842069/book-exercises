fn main() {
    // strings: collections of bytes with some useful methods

    // akshually, Rust only has ONE string type in the core language
    // the string slice 'str'
    // string literals = stored in the binary
    // string slices = reference to string data stored elsewhere
    // it's all UTF-8 encoded under the hood

    let s = "hello darkness".to_string();

    let mut half = "my old ".to_string();

    //using push_str() to append to a string
    half.push_str("friend!");

    // you can add strings using +...

    let space = " ".to_string();

    // let final_string = s + " " + &half;
    // let final_string = s + &space + &half;
    // println!("{}", final_string);

    // ...or using format!()

    let final_formatted_string = format!("{s}{space}{half}");
    println!("{}", final_formatted_string);
}

// let hello = String::from("السلام عليكم");
// println!("{}", hello);
// let hello = String::from("Dobrý den");
// println!("{}", hello);
// let hello = String::from("Hello");
// println!("{}", hello);
// let hello = String::from("שלום");
// println!("{}", hello);
// let hello = String::from("नमस्ते");
// println!("{}", hello);
// let hello = String::from("こんにちは");
// println!("{}", hello);
// let hello = String::from("안녕하세요");
// println!("{}", hello);
// let hello = String::from("你好");
// println!("{}", hello);
// let hello = String::from("Olá");
// println!("{}", hello);
// let hello = String::from("Здравствуйте");
// println!("{}", hello);
// let hello = String::from("Hola");
// println!("{}", hello);
