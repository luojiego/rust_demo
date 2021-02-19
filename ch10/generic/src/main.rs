use std::cmp::PartialOrd;
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T, // 实例化的时候，x 和 y 的类型一定要相同
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point1<T,U> {
    x: T,
    y: U, // 这样 x 和 y 不必非要类型一致
}

impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, other:Point1<V, W>) -> Point1<T,W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("the largest number is {}", result);

    let char_list = vec!['h', 'e', 'l', 'o', 'd', 'w', ];
    let result = largest(&char_list);
    println!("the largest char is {}", result);

    // Rust 的泛型与 C++ 有好多地方是相似的
    // 但其牛逼之处在于泛型代码是在编译器生成的，并非是 runtime
    // 所以几乎可以说是没有消耗
    
    let integer = Point { x: 5, y: 10};
    println!("x = {}", integer.x());
    // println!("integer.distance_from_origin(): {}", integer.distance_from_origin());
    // 无法编译, 未实现 Point<{integer}>: method not found in `Point<{integer}>`
    let float = Point {x: 1.0, y: 4.0};
    println!("float.distance_from_origin(): {}", float.distance_from_origin());
    // let float = Point { x: 1.0, y: 4}; // 无法编译通过，因为 4 会被编译器识别为 integer
    // 编译报错: expected floating-point number, found integer

    let both_integer = Point1 {x: 5, y: 10};
    let both_float = Point1 {x:1.0, y: 4.0};
    let integer_and_float = Point1 {x: 5, y:4.0};

    let p1 = Point1{x: 5, y:10.4};
    let p2 = Point1{x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3 = {:?}", p3);

}
