#[derive(Debug)]
struct Foo;

fn main()  {
    let a = Foo;

    // let closure = || {
    //     let b = a;
    // };

    let closure = move || {
        let b = a;
    };

    // println!("{:?}", a);
}