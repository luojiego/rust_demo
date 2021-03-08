#[derive(Debug)]
struct Foo(u32);

fn main() {
    let foo =  Foo(2048);
    let bar = foo;

    println!("Foo is {:?}", foo);
    //                      ^^^ value borrowed here after move
    println!("bar is {:?}", bar);
}
