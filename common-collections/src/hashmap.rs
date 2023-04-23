use std::collections::HashMap;

pub fn go_to_hashmapdotrs() -> () {

    let teams = vec![
        String::from("blue"),
        String::from("yellow"),
    ];

    let mut score_table = HashMap::new();

    score_table.insert(&teams[0], 10);
    score_table.insert(&teams[1], 12);


    // Iterating over HashMap -> 
    for (k, v) in &score_table {
        println!("{} -> {}", k, v);
    } 

    // Inserting unique keys in hashmap
    let data = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in data.split_whitespace() {
        let count = map.entry(word).or_insert(0); // Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.
        *count += 1;
    } 

    println!("{:?}", map);
}