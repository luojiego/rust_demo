use std::fs;

fn main() {
    // let mut i : u8 = 0;
    // for arg in std::env::args() {
    //     println!("index: {} args: {}", i, arg);
    //     i+=1;
    // }

    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 2 {
        println!("Usage: {:?} [url] [result markdown file path]", args[0]);
        return
    }

    // let url = "https://www.rust-lang.org/";
    // let output = "rust.md";
    // let url = "https://github.com/lesismal/arpc";
    // let output = "arpc.md";
    let url = &args[1];
    let output = &args[2]; 

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been save in {}", output);

}
