#[derive(Debug)]
pub struct Rectangle {
    width : u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

pub struct Guess {
    value: i32, 
}
 
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 { // wrong code !
            panic!("Guess value must be greater than 1!");
        } else if value > 100 {
            panic!("Guess value must be less than 100!");
        }
        Guess {value}
    }
}

#[cfg(test)]
mod tests {
    
    use crate::*;


    // #[test]
    // fn it_works() {
    //     assert_eq!(2+2, 4);
    // }

    // In rust a test fails when a function inside the test function panics!

    // #[test]
    // fn it_fails() {
    //     panic!("Something gone wrong...")
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let l = Rectangle {
            width : 8,
            height : 12,
        };

        let s = Rectangle {
            width : 5,
            height : 10,
        };

        assert!(l.can_hold(&s));
    }

    #[test]
    fn greeting_contains_name() {
        let res = greeting("Carol");
        assert!(
            res.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            res
        );
    }
    
    #[test]
    #[should_panic(expected = "Guess value must be less than 100!")]
    fn greater_than_100() {
        Guess::new(-2);
    }

    // Test who return an Result Type :
    #[test]
    fn it_works() -> Result<(), String>{
        if 2+2 == 5 {
            Ok(())
        } else {
            Err(String::from("Two plus Two is not equal to Four"))
        }
    }

}
