use std::{fs, thread::{self, JoinHandle}};
use anyhow::{anyhow, Result};
use serde_yaml::Value;
use toml;

/// 包装一下 JoinHandle，这样可以提供额外的方法
struct MyJoinHandle<T>(JoinHandle<Result<T>>);

impl<T> MyJoinHandle<T> {
    /// 等待 thread 执行完成（类型 await）
    pub fn thread_await(self) -> Result<T> {
        self.0.join().map_err(|_|anyhow!("failed"))?
    }
}

fn thread_read(filename: &'static str) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        let s = fs::read_to_string(filename)?;
        Ok::<_, anyhow::Error>(s)
    });
    MyJoinHandle(handle)
}

fn thread_write(filename: &'static str, content: String) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        fs::write(filename, &content)?;
        Ok::<_, anyhow::Error>(content)
    });
    MyJoinHandle(handle)
}

/// 使用线程来操作文件的读写
fn main() -> Result<()> {
    // println!("Hello, world!");
    let t1 = thread_read("./Cargo.toml");
    let t2 = thread_read("./Cargo.lock");

    let content1 = t1.thread_await()?;
    let content2 = t2.thread_await()?;

    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;
    println!("yaml1.len:{}", yaml1.len());
    println!("yaml2.len:{}", yaml2.len());

    let t3 = thread_write("/tmp/Cargo.yaml", yaml1);
    let t4 = thread_write("/tmp/Cargo.lock", yaml2);

    let yaml3 = t3.thread_await()?;
    let yaml4 = t4.thread_await()?;

    println!("yaml3.len:{}", yaml3.len());
    println!("yaml4.len:{}", yaml4.len());

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}
