use std::*;

pub fn slice_string(my_string: &String) {
    let hello = &my_string[0..5];
    let world = &my_string[6..11];

    println!("First slice: {}", hello);
    println!("Second slice: {}", world);
}