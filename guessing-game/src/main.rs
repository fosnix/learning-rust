use std::io;
use rand::Rng;

fn main() {

    let comp    = rand::thread_rng().gen_range(1..101);
    
    println!("Guessing Game");
    
    loop {
        println!("Please Enter your Guess!");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("failed to read guess");
        println!("Your guess is : {inp}");

        let inp : u32 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match inp.cmp(&comp) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => { 
                println!("You win!");
                break;
            }
    }

    }
}
