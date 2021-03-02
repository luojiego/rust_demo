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

	// reference
	{
		let s1 = String::from("hello");
		let len = calculate_length(&s1);
		println!("the length of '{}' is {}.", s1, len);
	}

	// mut refrence
	{
		let mut s1 = String::from("luojie");
		change_string(&mut s1);
		println!("s1 = {}", s1);
	}

	{
		let mut s1 = String::from("luojie");
		let s2 = &mut s;
		// let s3 = &mut s;
		// println!("{}, {}", s2, s3);
		println!("{}, {}", s1, s2);
	}

	{
		let s = String::from("hello world");
		let h = &s[0..5];
		let w = &s[6..11];
		println!("h: {} w: {}", h, w);
	}
}

fn task_ownership(s: String) {
	println!("task_ownership = {}", s);
}

fn make_copy(x: i32) {
	println!("make_copy = {}", x);
}

fn calculate_length(s: &String) -> usize {
	s.len()
}

fn change_string(s: &mut String) {
	s.push_str(" good job");
}

// fn dangle() -> &String {
// 	let s = String::from("hello");
// 	&s
// }
