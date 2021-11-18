use std::{cell::RefCell, fmt, sync::Arc, thread};

struct Lock<T> {
    locked: RefCell<bool>,
    data: RefCell<T>,
}

impl<T> fmt::Debug for Lock<T> 
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

// SAFETY: 我们确信 Lock<T> 很安全，可以在多个线程中共享
unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self { 
            data: RefCell::new(data),
            locked: RefCell::new(false),
        }
    }

    // !!! 重要
    // lock 的处理逻辑有问题，不能一个正常的流程
    // Refcell<bool> 不是线程安全的
    // atomic1 对其进行优化
    pub fn lock(&self, op: impl FnOnce(&mut T)) {
        // 如果没有拿到锁，就一直 spin
        while *self.locked.borrow() != false {} // **1

        // 拿到，赶紧加锁
        *self.locked.borrow_mut() = true; // **2

        // 开始干活
        op(&mut self.data.borrow_mut()); // **3

        // 释放锁
        *self.locked.borrow_mut() = false; // **4
    }
}

fn main() {
    let data = Arc::new(Lock::new(0));

    let data1 = data.clone();

    let t1 = thread::spawn(move || {
        data1.lock(|v|*v += 10);
    });

    let data2 = data.clone();
    let t2 = thread::spawn(move || {
        data2.lock(|v|*v += 10);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("data: {:?}", data);
}
