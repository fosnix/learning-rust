use std::fmt::Display;

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// String does need lifetimes because it has ownership assigned to it!
pub fn return_ownership() -> String {
    let s = String::from("Really long String!");
    s
}

// Automatic Lifetime Checking Rules -> 

// : Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

// 1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. 
//    In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

// 2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

// 3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. 
//    This third rule makes methods much nicer to read and write because fewer symbols are necessary.


// One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program. 
// All string literals have the 'static lifetime
pub const S: &'static str = "I have a static lifetime!";

pub fn longest_with_an_announcements<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}