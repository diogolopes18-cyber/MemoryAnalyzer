use std::*;
use std::io::stdin;

mod directories_list;
mod shadowing;
mod borrowing;
mod slice;

fn strings()  {
    println!("Define a string: ");

    //Define new string variable
    let mut my_string = String::new();
    stdin().read_line(&mut my_string).expect("Enter a string please");
    println!("{}", my_string);
}

fn main() {
    //Define new string
    let my_string = String::from("Hello World");

    shadowing::shadowing();
    borrowing::borrow();
    slice::slice_string(&my_string);
    strings();
}
