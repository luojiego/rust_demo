fn main() {
	let condition = true;
	let number = if condition {5} else {6};
	// let number = if condition {5} else {"six"};
	println!("number is {}", number);

	let mut counter = 0;
	let result = loop {
		counter += 1;

		if counter == 10 {
			break counter * 2;
		}
	};
	println!("the result is {}", result);

	let mut number = 3;
	while number != 0 {
		println!("{}!", number);
		number -= 1;
	}

	let a = [10, 20, 30, 40, 50];
	for element in a.iter() {
		println!("the value is: {}", element);
	}

	for number in (5..10).rev() {
		println!("{}!", number);
	}

	println!("fibnacci(3) = {}", fibnacci(3));
	println!("fibnacci(8) = {}", fibnacci(8));
	println!("LIFITOFF!!");
}

fn fibnacci(x:u32) -> u32 {
	if x == 1 {
		1 
	} else if x == 2 {
		1
	} else {
		fibnacci(x-1) + fibnacci(x-2)
	}
}
