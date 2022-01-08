use std::*;
use std::io::stdin;
pub mod basic_concepts;
pub mod cli_interface;

#[allow(dead_code)]
fn strings()  {
    println!("Define a string: ");

    //Define new string variable
    let mut my_string = String::new();
    stdin().read_line(&mut my_string).expect("Enter a string please");
    println!("{}", my_string);
}

#[allow(dead_code)]
fn basic_concepts() {
    let my_string = String::from("Hello World");
    basic_concepts::shadowing::shadowing();
    basic_concepts::borrowing::borrow();
    basic_concepts::slice::slice_string(&my_string);
    basic_concepts::enums::call();
}

fn main() {
    cli_interface::user_input::read_user_input();
}
