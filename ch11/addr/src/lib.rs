pub fn add_two(a :i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn prints_and_return_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

fn internal_adder(a: i32, b:i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // 设置忽略的测试用例
    // 运行该测试用例的命令
    // cargo test -- --ignored
    #[test]
    #[ignore]
    fn it_cant_works() {
        assert_eq!(2 + 3, 4);
    }

    use super::*;
    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Roger");
        assert!(result.contains("Roger"));
    }

    // 自定义用例失败的输出内容
    #[test]
    fn greeting_not_contains_name() {
        let result = greeting("Roger");
        let name = "luojie";
        assert!(
            result.contains("luojie"),
            "Greeting result \"{}\" not contains {}",
            result,
            name,
        );
    }

    // 用例对于 panic 的检测
    #[test]
    #[should_panic]
    fn test_err_index() {
        let v = vec![1,2,3];
        v[99];
    }

    // 没有 panic 发生
    #[test]
    #[should_panic]
    fn not_panic() {
        let v = vec![1,2,3];
        v[2];
    }

    // 用例对于 panic 输出文本的检测
    // 以下用例会失败，因为 expect 是 99，实际上结果是 100
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 99")]
    fn test_err_index2() {
        let v = vec![1,2,3];
        v[100];
    }

    // Result 的使用
    #[test]
    fn it_works_result() -> Result<(),String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("tow plus two does not equal four"))
        }
    }

    // Rust 测试用例 默认是 Parallel 并行的
    // 若测试用例有先后顺序的要求，需要设定 --test-threads = 1
    // 完整命令为 cargo test -- --test-threads=1

    // Rust 被测试函数有输出的问题
    // 使用 cargo test this 时
    // this_test_will_pass 并不会输出 "I got the value 4"
    // 但是 this_test_will_fail 会输出 "I got the value 8"
    // 若需要输出，加上参数 -- --show-output 即可
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_return_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_return_10(8);
        assert_eq!(4, value);
    }

    // 测试私有的函数
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
