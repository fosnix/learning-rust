// Here’s the scenario: Every so often, our t-shirt company gives away an exclusive, limited-edition shirt to someone on our mailing list as a promotion. 
// People on the mailing list can optionally add their favorite color to their profile. 
// If the person chosen for a free shirt has their favorite color set, they get that color shirt. 
// If the person hasn’t specified a favorite color, they get whatever color the company currently has the most of.

use std::{thread, time::Duration};

struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

impl Inventory {
    fn giveaway(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Red
        }
    }
}


// Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions!

// You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.
// Unlike functions, closures can capture values from the scope in which they’re defined.
// We’ll demonstrate how these closure features allow for code reuse and behavior customization

// Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do. 
// Type annotations are required on functions because the types are part of an explicit interface exposed to your users.
// Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario.

// the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables
// As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary.

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1.unwrap(), giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    };

    expensive_closure(3);

    // For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value.

    let return_x = |x| return x;
    return_x(String::from("Hi"));
    // return_x(5); // Error : Concrete Type -^---- Infered Above!


    // Closures can capture values from their environment in three ways, 
    // which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership.

    // The closure will decide which of these to use based on what the body of the function does with the captured values.

    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    
    let mut list2 = vec![1,2,3];
    let mut borrows_mutably = || list2.push(4);
    // println!("{:?}", list2); Error : Can't borrow immutably while mutable borrow in being used!
    borrows_mutably();
    println!("After calling mut borrowed closure!");

    // Moving the ownership of data to closure ->
    let list3 = vec![1,2,3];
    println!("Before defining closure: {:?}", list3);
    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();

}
