fn main() {

    // Variables are immutable by default in rust!
    let x = 5;
    // x = 10; // Error! 

    let mut y = 5; // variables can be explicitly declared as mutable!
    y = 5*2; // Ok!

    println!("x : {x}");
    println!("y : {y}");
    println!("Value of PI = {PI}");

    // Shadowing in rust ->  Used for transformation of value of immutable variables in rust by using the let keyword and with same variable name!
    let a = "     ";
    let a = a.len(); // can also change the type of the variable by using this approach!

    println!("Value of a is : {a}");

}

const PI : f64 = 3.141592653589793; // constant in rust!