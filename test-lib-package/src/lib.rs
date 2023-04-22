// A simple library-carte mimicing a restaurant's serving and hosting area!
// A module can have functions, constants, enums, structs and any other collections!
// By default the child module and every thing inside it is private to parent module, but the child module can see anything inside the parent module!  

mod front_end { // Parent-Module
    
    fn use_super_keyword_to_access_me() {}

    pub mod hosting { // Sub-Module
        pub fn add_to_waitlist() {
            println!("Added to Waitlist!");
        }
        pub fn seat_at_table() {}
    }

    pub mod serving { // Sub-Module
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}

        fn fix_incorrect_order() {
            cook_order();
            super::use_super_keyword_to_access_me(); // 'super' is used to pull a function from it's parent scope!
        }

        fn cook_order() {}
    }

}

// use crate::front_end::hosting; // absolute path!
use self::front_end::hosting; // relative path!

pub fn eat_at_restaurant() {
    // Absolute Path : 
    // crate::front_end::hosting::add_to_waitlist();
    
    // Relative Path :
    // front_end::hosting::add_to_waitlist();

    hosting::add_to_waitlist(); // 'use' keyword is used to import a path into the scope! 

}