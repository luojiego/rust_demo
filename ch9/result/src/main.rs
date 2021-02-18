use std::fs::{self,File};
use std::io::ErrorKind;
use std::io;
use std::io::Read;
fn main() {
    // test5();
    println!("read_username_from_file3: {:?}", read_username_from_file3());
}

fn test1() {
    // Result<T, E> 的用法
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn test2() {
    // 在文件找不到的时候创建 
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_err => {
                panic!("Problem opening the file: {:?}", other_err)
            }
        },
    };
}

fn test3() {
    // test2 更加简略的写法
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn test4() {
    let f = File::open("hello.txt").unwrap();
}

fn test5() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file () -> Result<String, io::Error> {
    // 传播 Error
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    // 简化版的 read_username_from_file
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    // 简化版的 read_username_from_file2
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    // 简化版的 read_username_from_file4
    fs::read_to_string("hello.txt")
}
