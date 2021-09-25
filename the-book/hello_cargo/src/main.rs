#[allow(unused_variables)] // 允许未使用的变量 一般不要加
#[allow(dead_code)]

// #[allow(dead_code)] // 
fn test() {

}

struct Type {
    pub a: i32,
}

pub trait Print{
    fn print(&self) {
        println!("default ");
    }
}

struct Data<T> {
    a: T,
    b: T,
}

impl<T> Data<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { a, b }
    }

    // pub fn add(&self) -> i32 { self.a + self.b}
}

impl Print for Data<i32> {
    fn print(&self) { println!("{{\n\ta:\t{},\n\tb:\t{}\n}}", self.a, self.b)}
}

impl Data<i32> 
// where 
//    T: i32,
{
    pub fn add(&self) -> i32 { self.a + self.b}
} 

fn main() {
    let arr = [0usize;8];
    println!("{:?}", arr);
    let _a = 3;
    println!("Hello, world!");

    let d = Data::<i32>::new(2,3);
    println!("{}", d.add());
    d.print();

    // let d1 = Data{a:"5.0", b:"10.0"};
    // d1.add();
    println!("{}", "hello ".to_string() + "world");

    let h = "hello ".to_string();
    let r;
    {
        let w = "world".to_string();
        r = h + &w;
    }
    // println!("{}", h);
    println!("{}", r);
}
