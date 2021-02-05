# ownership

## ownership 相关

同 go 一样，允许有多返回值，还是要注意** Rust 的返回值后面没有分号**

### move

```rust
{
	let s = String::from("hello");
	printString(s);
	println!("s = {}", s); // 编译不通过，因为 s 已经 move 到了 printString 中
}

{
    let s1 = String::from("s1");
    let s2 = s1;
    // println!("s1 = {}", s1);  // compile err: value borrowed here after move
}

fn printString(s String) {
	println!("s = {}", s);
}
```

上面代码直接手写的，可能编译不通过

### clone

```rust
{
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {} s2 = {}", s1, s2);
}
```

虽然实现了功能，但是 Rust 也明确表示了，clone 意味成成本特别高，大多数性能问题也来自于 clone

### copy

基本数据类型均是采用 copy 方式的，因为其占用空间和在编译期间可确定的大小，所以均使用的是 copy，不会 move，也不用使用 clone。

```rust
{
	let x = 3;
	make_copy(x);
	println!("after make_copy = {}", x);
}

fn make_copy(x: i32) {
	println!("make_copy = {}", x);
}

```

## reference

```rust
 // reference
 {
     let s1 = String::from("hello");
     let len = calculate_length(&s1);
     println!("the length of '{}' is {}.", s1, len);
 }

 fn calculate_length(s: &String) -> usize {
     s.len()
 }

```

使用引用，就解决了 s1 以参数传进 calculate_length 之后，所有权被移除的问题。 

### borrowing

在函数 calculate_length 函数中，函数参数属于借用，在函数作用域，并没有所有权，所以在离开作用域时，也不会调用 drop。

同样，在此函数中**并不能修改 s 的数据**，若需要修改，就要引出可变引用了。

### mut reference

**可变引用**

```rust
 // mut refrence
 {
     let mut s1 = String::from("luojie");
     change_string(&mut s1);
     println!("s1 = {}", s1);
 }
 
 fn change_string(s: &mut String) {
     s.push_str(" good job");
 }

```

#### 限制及 data race

**在特定作用域中的特定数据只能有一个可变引用**

```rust
{
    let mut s1 = String::from("luojie");
    let s2 = &mut s; // first mutable borrow occurs here
    let s3 = &mut s; // second mutable borrow occurs here
    println!("{}, {}", s2, s3);
}
```

无法编译，因为在特定作用域 s1 只有能一个可变引用。



这样限定的好处就是要在编译期间避免数据竞争，由以下三个行为造成：

1. 两个或更多指针同时访问同一个数据；
2. 至少有一个指针被用来写入数据；
3. 没有同步数据访问的机制；



### dangling refrences

**悬挂引用**

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```

编译报错，不允许这样的代码

```rust
fn return_string() -> String {
	let s = String::from("hello");
	s
}
```

所有权已转移，没有问题

## slice

有点类似 go 的切片，但是从根本上在避免不可预见的运行时问题。