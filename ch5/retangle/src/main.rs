#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	let rec = Rectangle {
		width: 30,
		height: 50,
	};

	println!("area: {}", area(&rec));
	println!("Rectangle: {:?}", rec);
	println!("Rectangle: {:#?}", rec);
}

fn area(r: &Rectangle) -> u32 {
	r.width * r.height
}
