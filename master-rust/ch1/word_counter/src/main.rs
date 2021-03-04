use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};
use std::env;

#[derive(Debug, Clone, Copy)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    fn increment(mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(self) {
        for (key, value) in self.0.iter() {
            println!("{}: {}", key, value);
        }
    }
}

// impl Clone for WordCounter {
//     fn clone(&self) -> Self {
//         
//     }
// }
// 
// impl Copy for WordCounter{
// 
// }

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = &arguments[1];
    println!("Processing file: {}", filename);

    let file = File::open(filename).expect("open file err");
    
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue
            } else {
                word_counter.increment(word)
            }
        }
    }
    word_counter.display();
}
