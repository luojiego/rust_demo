use std::panic;
fn main() {
    // println!("Hello, world!");
    let result = panic::catch_unwind(||{
        println!("hello!");
    });
    assert!(result.is_ok());
    let result = panic::catch_unwind(||{
        panic!("oh no!");
    });
    assert!(result.is_err());
    println!("panic captured: {:?}", result);
}
