use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let a: f32 = 290.1234567;
    println!("a: {}", a);
    println!("a: {:.7}", a);
    let b: f64 = 290.12345678901234567890;
    println!("b: {}", b);
    println!("b: {:.20}", b);
    
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    for received in rx {
        println!("Got: {}", received);
    }
}
