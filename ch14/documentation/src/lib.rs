//! # documentaion
//!
//! `documentation` is a collection of utilities to make performing certain
//! calculations more convenient.
/// Adds one to the number given.
/// 
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = documentation::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

pub fn add_one(i: i32) -> i32 {
    i + 1
}

// 使用 cargo doc 生成文档
// 使用 cargo doc --open 在本地打开文档，字体真的好有爱
