// This is a binary-crate in the package 'module-sys'

use module_sys::say_hello;

fn main() {
    say_hello::hello();
    say_hello::hello_world();
}   
