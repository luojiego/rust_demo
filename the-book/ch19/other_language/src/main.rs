extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 3;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    {
        add_to_count(3);
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}
