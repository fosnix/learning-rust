use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

pub fn error_handling_in_rust() -> () {
    // panic!("Hello, world!"); // Panics here and stops the exxecution of program!

    // Recoverable Errors in rust!

    let file_result = File::open("Hello.txt");

    // Multiple error handling using match expressions and Result Enum!
    let file_result = match file_result {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(ec) => panic!("Problem creating the file : {}", ec),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    println!("{:?}", file_result);

    let another_file = File::open("Hello.txt").expect("Some problem in opening this file!");
    println!("{:?}", another_file);
    
    
    let result_of_fun = read_username_from_file();
    println!("{:?}", result_of_fun);
    
}


// Error Propogation!

fn read_username_from_file() -> Result<String, io::Error> {

    let mut username = String::new();

    // let username_file_result = File::open("Hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(f) => f,
    //     Err(e) => return Err(e),
    // };


    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => return Ok(username),
    //     Err(e) => return Err(e),
    // }

    // A Shortcut for Propagating Errors: The ? Operator==>
    
    File::open("Hello.txt")?.read_to_string(&mut username)?;
    return Ok(username);
}