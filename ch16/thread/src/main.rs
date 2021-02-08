use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // 如果放在这个位置，将不会交替打印
    
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // 放在这个位置，只有线程结束之后，程序才会结束

    let v = vec![1,2,3];
    // let handle = thread::spawn(||{
    //     println!("here's a vec {:#?}", v);
    // });
    // 无法编译 may outlive borrowed value `v`
    //

    // 使用 move
    let handle = thread::spawn(move || {
        println!("here's a vec {:#?}", v);
    });

    // drop(v); // value used here after move
    // 使用了 move，则意味着 v 所有权已交给了 thread，则在主线程不能再操作 v
    handle.join().unwrap();
}
