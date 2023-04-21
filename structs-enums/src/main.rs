#[derive(Debug)]
// Defining a 'User' Struct : 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
// Defining a enum :
enum IPAddrType {
    V4(String),
    V6(String),
}


fn main() {

    // Immutable 'User' instance or object 'u1'!
    let u1 = User {
        active : true,
        username : String::from("Uday"),
        email : String::from("uday@email.com"),
        sign_in_count : 69,
    };

    // Shorthand syntax for structs!
    let u2 = User {
        active : false,
        username : String::from("Uday"),
        email : String::from("uday@email.com"),
        ..u1 // Else same as 'u1'!
    };

    let white_col = Color(255, 255, 255);
    let sun = AlwaysTrue;
    
    println!("u1 : {:?}", u1);
    println!("u2 : {:?}", u2);
    println!("white_col : {:?}", white_col);
    println!("sun : {:?}", sun);

    let localhost = IPAddrType::V6(String::from("127.0.0.1"));
    println!("localhost : {:?}", localhost);

}

#[derive(Debug)]
// Tuple Structs : 
struct Color(u32, u32, u32);

#[derive(Debug)]
// Unit like structs : 
struct AlwaysTrue;