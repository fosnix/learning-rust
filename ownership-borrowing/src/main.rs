fn main() {

    let s = "Hello";  // s is the owner of the string literal "hello"!
    println!("{}",s);

    let mut l_str = gives_ownership();
    println!("{}", l_str);
    l_str = takes_and_gives_back(s);

    println!("{}", l_str);
    println!("{}", l_str);

    // References are used to take a variable's data without affecting its ownership -> Knowns as Borrowing!

    let r = &l_str;
    println!("{}", r);

    let r1 =  &mut l_str;
    println!("{}", r1);

    // Rules of Referencing -> 

    // Let’s recap what we’ve discussed about references:

    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid. (References should not dangle!)

} // s is out of scope here hence droped from the memory!

fn gives_ownership() -> String {
    return String::from("I am your's now!"); // Return a value which can be assigned to any variables making it it's owner.
}

fn takes_and_gives_back(s: &str) -> String {
    return String::from(s); // takes the refrence from 's' and gives as a value to 'Str'.
}