fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("loop next val is {}", c);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("while next val is {}", c);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1,1);
    for _i in 2..n{
        let c = a + b;
        a = b;
        b = c;
        println!("for next val is {}", c);
    }
}

fn main() {
    println!("Hello, world!");
    // fib_loop(8);
    // fib_while(10);
    fib_for(15);
}
