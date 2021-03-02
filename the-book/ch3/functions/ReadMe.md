# 随便可见的 fn
## 函数命名
和 go 不一样，snake case

## 注意点

### 赋值语句

```rust
fn main() {
    println!("Hello, world!");
    other_function();
    let x = 5;
    let y = {
        let x = 3; // 不会覆盖 5
        x + 1 // 不能写分号!!!
    };
}
```

### x = y = 6

Rust 不支持这样写

### fn other_function(x,y : i32)

只能写成 x:i32, y:i32

### 函数返回值

- 没有 return；
- 也没有 ";"，加上分号编译器就哭给你看
- 返回值更加形象 "->"

```rust
fn plus(x:i32) -> i32 {
        x+1
}
```

