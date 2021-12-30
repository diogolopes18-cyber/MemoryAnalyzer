use std::path::Path;
use std::env;

//'a is a lifetime specifier
//It tells Rust that the struct is valid as long
//As the reference is valid
//It is used to ensure all borrows are valid

#[derive(Debug)]
struct File<'a> {
    size: usize,
    name: &'a String
}

fn get_file_size(filename: &String) -> File {
    let filesize = filename.len();
    let file = File { size: filesize, name: &filename };
    println!("Size: {:?} \nName: {:?}", file.size, file.name);
    return file;
}

pub fn read_user_input() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    if filename.len() > 0 {
        //Check if file exists
        if file_exists(&filename) == true {
            get_file_size(filename);
        }
    }
}

fn file_exists(name: &String) -> bool{
    Path::new(name).is_file()
}