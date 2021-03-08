// #[derive(Copy, Debug)]

#[derive(Copy, Clone, Debug)]
struct Dummy;

fn main() {
    let a = Dummy;
    let b = a; // 如果类型实现了 Copy，则从一个变量到另一个变量的赋值操作将隐式复制数据
    println!("{:#?}", a);
    println!("{:#?}", b);
}