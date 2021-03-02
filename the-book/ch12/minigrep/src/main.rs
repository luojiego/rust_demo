use std::process;
use std::env;
use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // 优化
    // println!("{:#?}", args);
    // let query = &args[1];
    // let filename = &args[2];

    // 优化1
    // let args: Vec<String> = env::args().collect();
    // let (query, filename) = parse_config(&args);
    // println!("Searching for {}", query);
    // println!("In file {}", filename);

    // 优化 2
    // let args: Vec<String> = env::args().collect();
    // let config = parse_config(&args);

    // 优化 3
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args);

    // 优化 4
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err|{
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // 优化 9
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // let contents = fs::read_to_string(config.filename)
    //     .expect("Something went wrong reading a file");

    // println!("With text: \n{}", contents);

    // 优化 5
    // run(config);

    // 优化 6
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// 优化1
// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
// 
//     (query, filename)
// }



// fn run(config: Config) {
//     let contents = fs::read_to_string(config.filename)
//         .expect("Something went wrong reading the file");
//     println!("With text:\n{}", contents);
// }



// 优化 2
// fn parse_config(args: &[String]) -> Config {
//    let query = args[1].clone(); 
//    let filename = args[2].clone(); 
// 
//    Config {query, filename}
// }