fn main() {
    use std::collections::HashMap;

    // initialize a hash map of runners to store their average lap timings
    let mut runners = HashMap::new();

    let joe = "Joe".to_string();
    let jill = "Jill".to_string();
    let jane = "Jane".to_string();
    let jack = "Jack".to_string();

    // insert joe and set his average to 10
    runners.insert(joe.clone(), 10);
    println!("DEBUG: there's only Joe");
    println!("\tthe current values are:\n\t{runners:?}");

    // insert other runners and set their default values
    runners.entry(jill.clone()).or_insert(5);
    runners.entry(jane.clone()).or_insert(5);
    runners.entry(jack.clone()).or_insert(5);
    println!("\nDEBUG: we initialized Jill, Jane and Jack with 5 minutes per lap");
    println!("\tthe current values are:\n\t{runners:?}");

    // hard update jill's average to another value
    println!("\nDEBUG: hard setting Jill's value to 20");
    println!("\tJill's average before: {:?}", runners.get(&jill).unwrap());
    runners.insert(jill.clone(), 20);
    println!("\tJill's average after: {:?}", runners.get(&jill).unwrap());

    // update current values based on old values
    println!(
        "\nDEBUG: runners have completed one lap
       their timings are:
       \tJill: 4
       \tJack: 10
       \tJane: 8
       \tJoe: 9"
    );
    println!("\nDEBUG: updating their values...");
    println!("\tvalues before updating:\n\t{runners:?}");

    // updating values using another hash map
    let mut new_values = HashMap::new();

    new_values.entry(jane.clone()).or_insert(8);
    new_values.entry(jill.clone()).or_insert(4);
    new_values.entry(jack.clone()).or_insert(10);
    new_values.entry(joe.clone()).or_insert(9);

    for (key, val) in new_values.iter() {
        let updater = runners.entry(key.clone()).or_insert(0);
        *updater = (*updater + val) / 2;
    }

    println!("\n\tvalues after updating:\n\t{runners:?}");
}
