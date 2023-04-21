fn main() {

    let x = 10;

    let some_number : Option<i32> = Some(5);

    let some_string = Some(String::from("Uday Tiwari"));

    let absent_number : Option<i32> = None;

    println!(" {:#?}, {:#?}, {:#?}", some_number, some_string, absent_number);

    // let sum = some_number + x; // Error -> {Mismatched Type!}

    let sum = some_number.unwrap_or(2) + x;
    println!("{}", sum);

    let added_one = plus_one(some_number);
    println!("{added_one}");

    // If-let syntax!
    if let Some(5) = some_number {
        println!("Five");
    }
}

fn plus_one(x : Option<i32>) -> i32 {
    match x {
        Some(i) => Some(i).unwrap() + 1,
        // None => 0, OR 
        _ => 0,
    }
}