#[derive(Debug)]
fn main() {
    {
        let v = vec![1,2,3];
        for i in &v {
            println!("{}", i);
        }
    }
    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        println!("v1: {:#?}", v);
        let mut v2 = Vec::new();
        v2.push("luo");
        v2.push("jie");
        println!("v2: {:#?}", v2);
    }

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        // let does_not_exist = &v[100]; // 越界会 panic
        let does_not_exist = v.get(100); // 使用 get 不会 core
        println!("does not exist {:#?}", does_not_exist);
    }
    {
        let mut v = vec![1,2,3,4,5];
        let first = &v[0];
        v.push(6);
        // println!("the first element is: {}", first);
        // 无法编译 因为 first 之后，push 可能导致 v 整体地址发生变化
        // 非常类似 go 的 list，append 之后，list 的地址可能会变化
    }
    {
        let mut v = vec![1,2,3];
        println!("origin data:");
        for i in &v {
            println!("\t {}", i);
        }

        for i in &mut v {
            *i *= 5;
        }

        println!("after *i *= 5");
        for i in &v {
            println!("\t {}", i);
        }
    }
    {
        // 存储不同类型的数据
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        // println!("row: {:#?}", row);
    }
}
