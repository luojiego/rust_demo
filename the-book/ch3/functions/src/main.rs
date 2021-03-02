fn main() {
    println!("Hello, world!");
	other_function();
	let x = 5;
	let y = {
		let x = 3;
		x + 1
	};
	println!("the value of x is: {}", x);
	println!("the value of y is: {}", y);
	// x = 33;
	// println!("the value of x is: {}", x);
	other_function1(5,6);

	// let mut a=b=6; // 不支持的写法
	println!("five return: {}", five());
	println!("plus(five()): {}", plus(five()));
}

fn other_function() {
	println!("我是 other_function");
}

fn other_function1(x:i32, y:i32) {
	println!("x is: {}", x);
	println!("y is: {}", y);
}

fn five() -> i32 {
	5
}

fn plus(x:i32) -> i32 {
	x+1
}
