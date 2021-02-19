fn main() {
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
