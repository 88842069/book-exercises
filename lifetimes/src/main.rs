// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = String::from("xyz");
//     let result = longest(string1.as_str(), string2.as_str());
//     println!("The longer string is: {result}");
// }
//
// fn longest<'somelifetime>(a: &'somelifetime str, b: &'somelifetime str) -> &'somelifetime str {
//     if a.len() > b.len() { a } else { b }
// }
//
struct ImportantExcerpt<'a> {
    part: &'a str,
}
//
// fn return_excerpt(story: ImportantExcerpt) -> &'a str {
//     let first_sentence = novel.split('.')
// }

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
