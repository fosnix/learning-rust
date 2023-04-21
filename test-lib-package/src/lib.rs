// A simple library-carte mimicing a restaurant's serving and production area!
// Every module in Rust is private by default, you can make it public with 'pub' keyword!


mod front_end { // Parent-Module

    pub mod hosting { // Sub-Module
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving { // Sub-Module
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute Path : 
    // crate::front_end::hosting::add_to_waitlist();
    
    // Relative Path :
    front_end::hosting::add_to_waitlist();`:wq

}



