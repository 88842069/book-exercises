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

    // can be improved using hash maps
    let val = vec![4, 10, 8, 9];
    let names = ["Jill", "Jack", "Jane", "Joe"];

    let mut count = 0;

    for n in names {
        let tmp = n.to_string();
        let updater = runners.entry(tmp.clone()).or_insert(0);
        *updater += val[count];
        *updater /= 2;
        count += 1;
    }

    println!("\n\tvalues after updating:\n\t{runners:?}");
}

/* figuring out how to update values

println!("Jill's average before: {:?}", runners.get(&jill).unwrap());

let curr_value = runners.get(&jill).unwrap();
println!("\t{curr_value}");

let new_value = 4;
println!("\t{new_value}");

let final_value = (curr_value + new_value) / 2;
println!("\t{final_value}");

runners.insert(jill.clone(), final_value);

let updater = runners.entry(jill.clone()).or_insert(0);
*updater += new_value;
*updater /= 2;

println!("Jill's average after: {:?}", runners.get(&jill).unwrap());

*/
