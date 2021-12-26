use std::fs;
use std::*;

pub fn directory_name() {
    //unwrap returns the type of the function in case of success
    //If there is an error it will return an error
    let directory = String::from("./");

    let path = fs::read_dir(&directory).unwrap();

    for paths in path {
        println!("Name: {}", paths.unwrap().path().display());
    }
}