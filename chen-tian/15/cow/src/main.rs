use std::borrow::Borrow;
use std::borrow::Cow;
use url::Url;
fn main() {
    println!("Hello, world!");
    let s = "hello, world".to_owned();
    let r1: &String = s.borrow();
    let r2: &str = s.borrow();
    println!("r1: {:p}, r2: {:p}", r1, r2);

    query_cow();

    serde();
}

fn query_cow() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = url.query_pairs();
    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    println!("key: {}, value: {}", k, v);

    k.to_mut().push_str("_lala");
    print_pairs((k, v));

    print_pairs(pairs.next().unwrap());
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed {}", v),
        Cow::Owned(v) => format!("Owned {}", v)
    }
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>, 
    age: u8,
}

fn serde() {
    let input = r#"{"name": "Tyr", "age": 18}"#;
    let user: User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Borrowed(x) => println!("borrowed {}", x),
        Cow::Owned(x) => println!("owned {}", x),
    }
}