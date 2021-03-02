fn main() {
    println!("Hello, world!");
	let guess: u32 = "42".parse().expect("Not a number!");
	// 以下的写法会导致编译问题
	// let guess = "42".parse().expect("Not a number!");

	// let u: u8 = 256; // 编译不过，提示越界，不清楚是不是文档没有正常同步
	// println!("u = {}", u);

	let n: u32 = 1_321_123; // 这样的表示方式很有创新哈，很 6
	println!("n = {}", n);

	let tup: (i32, f64, u8) = (500, 6.4, 1);
	println!("tup 0 = {}", tup.0);

	let tup1 = (5,3.2,2);
	let (x,y,z) = tup;
	println!("y is = {}", y);
}
