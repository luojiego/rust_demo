use std::{thread, time::{self, Duration, SystemTime, UNIX_EPOCH}};

fn main() {

}

#[test]
fn test_timestamp() {
    let s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", s.as_secs());
}

#[test]
fn test_time_elapsed() {
    let ten_millis = Duration::from_secs(10);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);
}

#[test]
fn test_thread2() {
    let v = vec![1,2,3,4,5,6,7,8,9];
    let handler = thread::spawn(move||{
        for (k, v) in v.iter().enumerate() {
            println!("thread v k: {}, value: {}", k, v);
            thread::sleep(Duration::from_micros(10));
        }
    });
    handler.join().unwrap();
}

#[test]
fn test_thread1() {
    let handler = thread::spawn(||{
        for i in 1..10 {
            println!("thread {}", i);
            thread::sleep(Duration::from_micros(10))
        }
    });    
    for i in 1..10 {
        println!("main thread {}", i);
        thread::sleep(Duration::from_micros(10));
    }
    handler.join().unwrap();
}
