fn main() {
    let static_lifetime_variable: &'static str = "I have a static lifetime.";

    let str1 = String::from("abcd");
    let str2 = "def";
    let result = longest(str1.as_str(), str2);
    println!("the longest string is: {}", result);

    // 错误示范
    // let result2;
    // {
    //     let str2 = String::from("def");
    //     result2 = longest(str1.as_str(), str2.as_str());
    //     //                               ^^^^ borrowed value does not live long enough
    // }
    // println!("the longest string is: {}", result2);
    
    let result2;
    {
        let str2 = "defghi";
        result2 = longest(str1.as_str(), str2);
    }
    println!("the longest string is: {}", result2);

    let result3;
    {
        let str2 = "def";
        result3 = longest(str1.as_str(), str2);
    }
    println!("the longest string is: {}", result3);

    // String::from("abc") 和 ”abc“ 的区别需要研究一下

    let novel = String::from("Call me. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i: {:#?}", i);
    println!("i.announce_and_return_part(\"HaHa...\"): {}", i.announce_and_return_part("Haha"));

    println!("first_word(\"luo jie\"): {}", first_word("luo jie"));

    println!("static lifetime: {}", static_lifetime_variable);
}

// lifetime elison 不用声明 lifetime
fn first_word(s: &str) -> &str {
    // static 但是为什么不能在此打印 static_lifetime_variable
    // println!("static lifetime: {}", static_lifetime_variable);

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 错误示范
// fn longest(x: &str, y: &str) -> &str {
// //            ----     ----     ^expected named lifetime parameter
//     if x.len() > y.len() {
//         x 
//     } else {
//         y
//     }
// }

// 正确 lifetime 示范
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else {
        y
    }
}

// 正确
fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// // 错误 返回值并且 lifetime 和 x 不同
// fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
//     String::from("luojie").as_str()
//   //----------------------^^^^^^^^^
//   //returns a value referencing data owned by the current function
// }

// Lifetime Annotations in Struct Definitions
