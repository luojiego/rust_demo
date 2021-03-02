pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        return format!("read more from {} ...", self.summarize_author());
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 未覆盖默认的 trait
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{} by {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("tweet summarize(): {}", tweet.summarize());

    let article = NewArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };
    println!("article summarize(): {}", article.summarize());

    notify(&tweet);
    // 编译不能通过
    // let n = 3;
    // notify(&n); // the trait `Summary` is not implemented for `{integer}`

    notify1(&article);
}

pub fn notify(item: &impl Summary) {
    println!("breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify1<T: Summary> (item: &T) {
    println!("breaking news! {}", item.summarize());
}

pub fn notify2(item: &impl Summary, item2: &impl Summary) {

}

pub fn notify3<T: Summary>(item: &T, item2: &T) {

}

use std::fmt::Display;
// 同 C 和 C++ 一样，可以在代码中间 include

pub fn notify4<T: Summary + Display>(item: &T) {

}

pub trait Debug {

}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    3
}

fn some_function1<T,U> (t: &T, u: &U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug 
{
    5
}

fn return_summarize() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }   
}

// 想使用不同类型返回，但是这个代码目前还不能编译
// fn return_summarize(b: bool) -> impl Summary {
//     if b {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         } 
//     } else {
//         NewArticle {
//             headline: String::from("headline"),
//             location: String::from("location"),
//             author: String::from("author"),
//             content: String::from("content"),
//         }
//     }
// }
