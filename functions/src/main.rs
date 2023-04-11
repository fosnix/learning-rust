fn main() {

    println!("{:?}", greet("Uday"));
    
    let sum = {
        add_two(5, 4) // Use of semicolon makes this expression a statement!
    };

    println!("{:?}", sum);

}

// Statements vs Expressions (Rust is a Expressions Based Language) -> 

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

// Syntax of a function -> 

// fn func_name(param1: type , param2 : type, ...) -> return_type {
//     code ...
// }

fn add_two(a:i32, b:i32) -> i32 {
    return a+b;
}

fn greet(name:&str) -> String {
    return String::from(format!("Hello, {name}"));
}