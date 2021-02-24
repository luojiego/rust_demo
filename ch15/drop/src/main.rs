struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };

        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };

        println!("CustomSmartPointers created.");
    }
    {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        println!("CustomSmartPointers created.");
        // _c.drop(); // 提前释放
        drop(_c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}
