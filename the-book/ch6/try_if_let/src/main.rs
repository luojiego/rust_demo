fn main() {
    // if let 是简化版本的 match
    {
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }
    }

    // 使用 if let 优化成下面的代码
    {
        let some_u8_value = Some(0u8);
        if let Some(3) = some_u8_value {
            println!("three");
        } else {
            println!("not eq 3");
        }
    }
}
