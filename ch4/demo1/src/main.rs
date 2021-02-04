fn main() {
	let mut s = String::from("hello");
	s.push_str(" world!");
	println!("s = {}", s);
	{
		let s1 = String::from("s1");
		println!("s1 = {}", s1);
	}
	// println!("s1 = {}", s1); // 无法编译通过
	{
		let s1 = String::from("s1");
		let s2 = s1;
		// println!("s1 = {}", s1);  // compile err: value borrowed here after move
	}
	{
		let s1 = String::from("hello");
		let s2 = s1.clone();
		println!("s1 = {} s2 = {}", s1, s2);
	}

	{
		let s1 = String::from("hello");
		task_ownership(s1);
		// println!("s1 = {}", s1);
		let x = 3;
		make_copy(x);
		println!("after make_copy = {}", x);
	}
}

fn task_ownership(s: String) {
	println!("task_ownership = {}", s);
}

fn make_copy(x: i32) {
	println!("make_copy = {}", x);
}
