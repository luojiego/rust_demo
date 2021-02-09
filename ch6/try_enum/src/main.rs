fn main() {
    {
        // 非常常规的 enum 用法
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }

        #[derive(Debug)]
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let localhost = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        println!("localhost: {:#?}", localhost);
        println!("loopback: {:#?}", loopback);
    }
    {
        // 骚操作 1
        #[derive(Debug)]
        enum IpAddr {
            V4(String),
            V6(String),
        }
        let localhost = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));
        println!("localhost: {:#?}", localhost);
        println!("loopback: {:#?}", loopback);
    }

    {
        // 骚操作 我还可以再骚一点哦
        #[derive(Debug)]
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let localhost = IpAddr::V4(127,0,0,1);
        let loopback = IpAddr::V6(String::from("::1"));
        println!("localhost: {:#?}", localhost);
        println!("loopback: {:#?}", loopback);
    }

    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Write(String),
            ChangeColor(u8, u8, u8, u8),
        }

        impl Message {
            fn call(&self) {
                println!("哇偶");
            }
        }

        let m = Message::Write(String::from("hello world"));
        m.call();
    }

    {
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;

        println!("some_number: {:#?}", some_number);
        println!("some_string: {:#?}", some_string);
        println!("absent_number: {:#?}", absent_number);
    }
}
