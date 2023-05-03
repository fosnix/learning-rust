// A trait defines functionality a particular type has and can share with other types. 
// We can use traits to define shared behavior in an abstract way. 
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.

//A type’s behavior consists of the methods we can call on that type. 
// Different types share the same behavior if we can call the same methods on all of those types. 
// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

use std::fmt::Display;

pub struct NewsArticle {
    
    pub author : String,
    pub headline : String,
    pub content : String,

}

impl Summary for NewsArticle { 
    // Uses Default Implementation for summarize() !

    fn whois_auth(&self) -> String {
        return format!("From {}.", self.author);
    }
}

pub struct Tweet {

    pub username : String,
    pub content : String,
    pub is_reply : bool,
    pub is_retweet : bool,

}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}, by {}", self.content, self.username);
    }

    fn whois_auth(&self) -> String {
        return format!("Author of the tweet : {}", self.username);
    }
}

pub trait Summary {

    fn whois_auth(&self) -> String;

    // Shared Methods Here :
    fn summarize(&self) -> String {
        // Default Implementation here :
        return format!("Read more from {} ...", self.whois_auth());
    }
}

// Traits as params :
// pub fn notify(item : &impl Summary) -> () {
//     println!("Breaking News : {}", item.summarize())
// }

//      -^----- Syntatic Sugar for this : Trait Bound Syntax
pub fn notify<T: Summary>(item: &T) -> () {
    println!("Breaking News : {}", item.summarize())
}

// Trait Bound with where clause :
// pub fn some_function<T, U>(t: &T, u: &U) -> i32 where T: Display + Clone, U: Clone + Debug {
//     todo!();
// }

// Returning types the on which the trait is implemented :
//However, you can only use 'impl Trait' if you’re returning a single type.

pub fn returns_summarizeable() -> impl Summary {
    return Tweet {
        username : String::from("@udayofc"),
        content : String::from("Learning Rust!"),
        is_reply : false,
        is_retweet : false,
    };
} 

// Using Trait Bounds to Conditionally Implement Methods ;
// By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

pub struct Pair<T> {
    x : T,
    y : T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait. 
//Implementations of a trait on any type that satisfies the trait bounds are called 'blanket implementations'

// Example ->

// impl<T: Display> ToString for T {
//     // --snip--
// }