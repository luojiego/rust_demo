// 由于 hashmap 使用的比较少
// 所以要使用 use 来引用
// 另外也没有类似 vec![] 这样的宏来创建 HashMap
use std::collections::HashMap;

// 但是 std 不需要在 Cargo.toml 文件中单独进行配置
// 虽然没有在 prolude 中，但是由于是标准库，所以不需要在 Cargo.toml 的 dependencies 中写
fn main() {
    let mut h = HashMap::new();
    h.insert("罗杰", 32);
    h.insert("刘婷", 18);
    println!("{:?}", h);

    h.entry("刘婷").or_insert(19);
    h.entry("aoliao").or_insert(1);
    println!("{:?}", h);

    h.remove("刘婷");
    println!("after remove: {:?}", h);

    counter_words();
}

fn counter_words() {
    let s = String::from("hello world good world");
    let mut h = HashMap::new();
    for word in s.split_whitespace() {
        // 这里面的 count 很神奇哦，or_insert 返回的是 &'a mut V
        let count = h.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:#?}", h);
}