fn main() {
    let first = "Luo";
    let first_string = first.to_string();
    let second = "Jie";
    let name = first_string + " " + second;
    // let name = first + second // 不能对 &str + &str
    // println!("{} {}", first, first_string);
    println!("{}", name);

    // 显然 format 比 "+" 更方便
    // + 第一个元素必须是 String 类型
    // 但是 format 没有这个限制
    let name = format!("{} {}", first, second);
    println!("format: {}", name);

    // 那么问题来了，format! 的效率和 + 的效率如何对比呢？
    // 毕竟在 Go 中 fmt.Sprintf 的效率是远低于 + 的，特别在高频操作语句中

    // 研究一下 String 的 push_str 和 push 的操作
    let mut name = String::from("Roger");
    name.push_str(" push_str ");
    let address = String::from("Xi'an");
    name.push_str(&address);
    println!("name: {}", name);
    name.push('@');
    println!("name: {}", name);
}
