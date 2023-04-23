mod vec;
mod strings;
mod hashmap;

use vec::SpreadSheetCell;
use strings::go_to_stringsdotrs;
use hashmap::go_to_hashmapdotrs;

fn main() {
    // let mut v : Vec<i32> = Vec::new(); OR
    let mut v = vec![];
    v.push(1); // Mutable References to 'v'!
    v.push(2);
    v.push(3);

    println!("v = {:?}", v);

    // Unsafe way to access an element in vector ->
    println!("Element at index(2) = {:?}", &v[2]); // <- immutable refernece to 'v'!

    // Safe way to access an element in vector ->
    let e: &i32 = match v.get(2) {
        Some(ele) => ele,
        None => &-1,
    };

    println!("Element at index(2) = {:?}", e);

    // Iterating over vector ->
    for i in &v {
        println!("{i}");
    }

    // Iterating and mutating the vector ->
    for i in &mut v {
        *i += 1;
        println!("{i}");
    }

    println!("{:?}", v);

    let row = vec![
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Float(std::f32::consts::PI),
        SpreadSheetCell::Text(String::from("This is PI")),
    ];

    match &row[0] {
        SpreadSheetCell::Int(i) => println!("This is a Int variant of Value = {i}"),
        _ => println!("Not a Int variant"),
    };

    go_to_stringsdotrs();
    go_to_hashmapdotrs();
    
}
