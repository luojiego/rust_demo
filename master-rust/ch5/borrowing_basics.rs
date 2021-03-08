// #[derive(Debug)]
#[derive(Debug)]
struct Foo{items: u32}

// struct Foo(u32);

fn main() {
    let foo = Foo{items: 32};
    let bar = &foo;

    println!("Foo is {:?}", foo);
    println!("Bar is {:?}", bar);
}