use std::fs;
use std::*;
use std::path::Path;
use filesize::PathExt;

pub fn directory_name() {
    //unwrap returns the type of the function in case of success
    //If there is an error it will return an error
    let directory = String::from("./");
    check_memory(&directory);
    let path = fs::read_dir(&directory).unwrap();

    for paths in path {
        println!("Name: {}", paths.unwrap().path().display());
    }
}

fn check_memory(directory_name: &String) -> std::io::Result<u64> {
    let path = Path::new(&directory_name);

    //The '?' operator is a error handler
    //It returns the result if Ok
    //If not, it returns the error
    let size = path.size_on_disk();
    return size;
}