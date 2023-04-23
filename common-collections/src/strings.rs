pub fn go_to_stringsdotrs() -> () {

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!" );
    let s3 = format!("{}{}", s1, s2); // Format does not takes ownership of the interpolated strings!

    println!("{s3}");

    // UTF-8 encoded strings are used in rust!

    let emojies = String::from("😂❤️👌⚡");

    // byte stream of string 'emojies'
    for b in emojies.bytes() {
        println!("{b}");
    }

    // char stream of string 'emojies'
    for c in emojies.chars() {
        println!("{c}");
    }
}