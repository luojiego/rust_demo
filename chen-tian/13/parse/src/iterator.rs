struct Fibnacci {
    a: i32,
    b: i32, 
    cur: i32,
    total: i32,
}

impl Fibnacci {
    fn new(total: i32) -> Self {
        Self {
            a: 0, 
            b: 0, 
            cur: 0, 
            total,
        }
    }
}

impl Iterator for Fibnacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.total {
            return None;
        }

        if self.a == 0 {
            self.a = 1;
        } else {
            let a = self.a; 
            self.a = self.a + self.b; // 1 2
            self.b = a; 
        }
        self.cur += 1;
        Some(self.a)
    }
}

#[test]
fn test_fibnacci() {
    let f = Fibnacci::new(10);
    for iter in f {
        println!("{}", iter);
    }
}