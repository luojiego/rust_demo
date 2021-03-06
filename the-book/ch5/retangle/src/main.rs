use rectangle::Rectangle;
fn main() {
	let rec = Rectangle {
		width: 30,
		height: 50,
	};

    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rec3 = Rectangle {
        width: 40,
        height: 60,
    };
	println!("area(&rec): {}", area(&rec));
	println!("rec.area(): {}", rec.area());
	println!("rec: {:?}", rec);
	println!("rec2: {:?}", rec2);
    println!("rec.can_hold(rec2): {}", rec.can_hold(&rec2));
    println!("rec.can_hold(rec3): {}", rec.can_hold(&rec3));
	println!("rec: {:#?}", rec);
    println!("Rectangle::square(3): {:#?}", Rectangle::square(3));

}

fn area(r: &Rectangle) -> u32 {
	r.width * r.height
}
