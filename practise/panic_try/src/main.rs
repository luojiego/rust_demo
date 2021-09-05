use std::io;
// use std::fs::File;
use std::fs;
fn main() {
    // let s = read_name_from_file();
    // match s {
    //     Ok(name) => println!("the name is {}", name),
    //     Err(e) => println!("get name err: {}", e),
    // }

    if let Ok(name) = read_name_from_file() {
        println!("the name is {}", name);
    }
}

fn read_name_from_file() -> Result<String, io::Error> {
    // let mut name = String::new();
    // File::open("hello.txt")?.read_to_string(&mut name)?;
    // Ok(name)
    fs::read_to_string("hello.txt")
}