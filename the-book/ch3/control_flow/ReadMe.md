# 控制流 control flow
## 三目运算符
```rust
	let num = if condition { 5 } else { 6 };
```

注意：**和 C 类似，不允许 if 和 else 返回不相同的类型**

## 循环

### loop

可以使用 break 返回一些数据

### while

跟其它语言的 while 并没有多少区别

### for

没有多少特殊的东西，但是使用 索引 遍历数组，即不安全，效率又低；

```rust
        let a = [10, 20, 30, 40, 50];
        for element in a.iter() {
                println!("the value is: {}", element);
        }

```

