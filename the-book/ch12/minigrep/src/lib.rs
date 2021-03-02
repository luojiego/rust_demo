use std::error::Error;
use std::fs;
use std::env;
// 优化7
// 从 main.rs 把 struct 的定义和 run 函数迁移出来

// 优化 2
// 优化 3
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// 优化 3
impl Config {
    // 优化 4 错误参数
    // fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         panic!("not enough arguments");
    //     }
    //     let query = args[1].clone(); 
    //     let filename = args[2].clone(); 
    //     Config {query, filename}
    // }

    // 优化 9
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     // 优化 4 错误参数
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }

    //     let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    //     let query = args[1].clone(); 
    //     let filename = args[2].clone(); 
    //     Ok(Config {query, filename, case_sensitive})
    // }

    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // 优化 4 错误参数
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive})

        // powershell 命令行测试
        // $env:CASE_INSENSITIVE=1; cargo run to poem.txt
    }

}

// 优化 6
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // println!("With text:\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
} 

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 优化 10
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 优化 8
// 增加大小写不敏感
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    // 优化 10
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}