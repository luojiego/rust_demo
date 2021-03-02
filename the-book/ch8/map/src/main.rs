use std::collections::HashMap;
fn main() {
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 50);
        scores.insert(String::from("Yellow"), 100);

        println!("scores{{:?}}: {:?}", scores);
        println!("scores{{:#?}}: {:#?}", scores);

        // 多次使用 insert，现有值会直接被覆盖
        scores.insert(String::from("Yellow"), 250);
        println!("after re-insert scores{{:#?}}: {:#?}", scores);

        scores.entry(String::from("Yellow")).or_insert(300);
        scores.entry(String::from("Red")).or_insert(150);
        // Yellow 并没有变成 300, 因为 Yellow 已经存在
        // 因为 Red 不存在，所以插入了 Red 并设置了其值为 150
        println!("after entry scores{{:#?}}: {:#?}", scores);
    }

    {
        // let teams = vec![String::from("Blue"), String::from("Yellow")];
        let teams = vec!["Blue", "Yellow", "1", "2"];
        // 使用 String::from 或者 "" 都能正常工作
        let init_scores = vec![50, 100, 150];
        // 若两个 vector 长度不相同的时候，最终的 map 长度也就是长度最小的 map
        let mut scores: HashMap<_,_> = 
            teams.into_iter().zip(init_scores.into_iter()).collect();

        // 同 Go 一样，Rust 输出 hash 也是无序的
        println!("zip scores{{:?}}: {:?}", scores);
    }

    {
        // 所有权相关
        let key = "luojie";
        let value = String::from("男");
        let mut map = HashMap::new();
        map.insert(key, value);
        // map.insert(key, &value);
        println!("{:?}", map);
        println!("key: {}", key);
        // 以下代码不能编译，value borrowed here after move
        // 除非 insert 的时候是 map.insert(key,&value);
        // println!("value: {}", value);

    }

    {
        // get 
        // 若有结果则返回 Some(T), 若没有值，则返回 None
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 50);
        scores.insert(String::from("Yellow"), 100);
        println!("get \"Bule\": {:?}", scores.get(&String::from("Blue")));
        println!("get \"Yellow\": {:?}", scores.get("Yellow"));
        println!("get \"Red\": {:?}", scores.get("Red"));

        // 遍历
        for (key, value) in &scores {
            println!("key: {} \t value: {}", key, value);
        }
    }

    {
        // 非常典型的使用 hash 记录单词数量的程序
        // 很明显 C++ 的使用法更简单，因为有默认值的存在，可以直接 ++
       	let text = "hello world wonderful world";

		let mut map = HashMap::new();

		for word in text.split_whitespace() {
			let count = map.entry(word).or_insert(0);
			*count += 1;
		}

		println!("world count: {:?}", map); 
    }

}
