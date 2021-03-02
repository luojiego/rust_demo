fn main() {
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|  x + 1 ;

    println!("add_one_v1(1) = {}", add_one_v1(1));
    println!("add_one_v2(1) = {}", add_one_v2(1));

    let example_closure = |x: String| x;
    println!("example_closure(String::from(\"Roger\")) = {}", example_closure(String::from("Roger")));

    // 闭包可以使用外部的变量，但是函数不行
    let x = 4;
    let equal_to_x = |z: i32| z == x;

    let y = 4;
    assert!(equal_to_x(y));

    {
        let x = vec![1,2,3];
        let equal_to_x = move |z: Vec<i32>| z == x;
        // println!("x: {:?}", x); 在闭包中 x 已经 move 了，所以在此处无法使用
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }
}
