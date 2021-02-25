use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //      --- value moved here
        // send 的时候 val 就已经不在了
        // println!("val is {}", val);

        // 因为前面发送的是 String 后面不能发送 integer
        // 注释掉 let val ... 和 tx.send ... 
        // 移除掉下面三行的注释，可以编译，因为 integer 和 String 的实现并不一致
        // let x = 3;
        // tx.send(x).unwrap();
        // println!("val is {}", x);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
