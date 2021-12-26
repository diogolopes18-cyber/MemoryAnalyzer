use std::*;
use std::io::stdin;
mod directories_list;
mod basic_concepts;

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

    basic_concepts::shadowing::shadowing();
    basic_concepts::borrowing::borrow();
    basic_concepts::slice::slice_string(&my_string);
    //strings();



    match choice.trim() {
        "yes" => directories_list::directory_name(),
        "no" => println!("Exiting"),
        _ => {}
    }

}
