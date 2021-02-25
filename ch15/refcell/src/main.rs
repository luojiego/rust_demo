pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where
    T: Messager, 
{
    pub fn new(messager: &T, max:usize) -> LimitTracker<T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MockMessenger {
        fn send(&self, message: &str) {
            // self.send_messages.push(String::from(message));
            // 不使用 RefCell 的情况下，无法编译通过
            self.send_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_send_an_over_75_percent_warning_message(){
        let mock_messager = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messager.send_messages.borrow().len(), 1);
    }
}

fn main() {
}
