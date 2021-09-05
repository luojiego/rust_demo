fn main() {
    {
        let mut v = Vec::new();
        // 若没有 v.push("3") 上一句的代码是错误的 Vec::new 返回的是 Vec<T>
        // 只有 push 了之后，v 的类型才会被推测出来，即是 Vec<&str>
        v.push("3");
    }

    {
        // 使用 vec! 来初始化 Vector
        let v = vec![1,2,3,4];
        println!("{:#?}", v);

        // 注意事项 match 语句
        // 只有一句的时候，要加上 ","
        // 若是多行，用了 "{}"，可以不添加 ","
        let index3 = v.get(3);
        match index3 {
            Some(n) => {
                println!("index 3 is {}", n)
            },
            None => println!("no index 3 "),
        }

        // if let 表达式
        if let Some(3) = v.get(2) {
            println!("第二个元素是 3")
        }

        if let None = v.get(100) {
            println!("没有第 100 个元素")
        }

        /*
        注意顺序，看了一遍代码，自己写不出来，挺烦的
        if let v.get(10) = Option<None> {
            println!("no index 10 ")
        }*/
    }
}
