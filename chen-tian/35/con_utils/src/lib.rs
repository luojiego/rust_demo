use std::{collections::VecDeque, sync::{Arc, Mutex}};

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "hello world");
    }    

    #[test]
    fn multiple_senders_should_work() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();

        let t = thread::spawn(move||{
            s.send(1).unwrap();
        });

        let t1 = thread::spawn(move||{
            s1.send(2).unwrap();
        });

        let t2 = thread::spawn(move||{
            s2.send(3).unwrap();
        });

        for handle in [t, t1, t2] {
            handle.join().unwrap();
        }

        let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap];
        result.sort();
        assert_eq!(result, [1, 2, 3]);
    }



}

