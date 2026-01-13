use std::collections::HashMap;

fn main() {
    use std::collections::HashMap;

    let mut capitals = HashMap::new();

    capitals.insert(String::from("Tokyo"), 43_000_000);
    capitals.insert(String::from("Beijing"), 62_000_000);
    capitals.insert(String::from("South Korea"), 29_000_000);

    for (key, value) in &capitals {
        println!("{key}: {value}")
    }

    let test1 = String::from("Tokyo");
    let test2 = String::from("London");

    let passing_test = get_population(capitals.clone(), test1);
    let failing_test = get_population(capitals, test2);

    println!(
        "\npassing test: {:?}\nfailing test: {:?}",
        passing_test, failing_test
    );
}

fn get_population(capitals: HashMap<String, u32>, key: String) -> u32 {
    let population = capitals.get(&key).copied().unwrap_or(0);
    population
}
