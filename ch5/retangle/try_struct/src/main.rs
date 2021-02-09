#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 定义时各成员的变量顺序随意
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        sign_in_count: 1,        active: true,
    };
    println!("user1: {:#?}", user1);

    let mut user2 = User {
        email: String::from("someone01@example.com"),
        username: String::from("someone01"),
        sign_in_count: 1,
        active: true,
    };
    println!("user2: {:#?}", user2);
    user2.username = String::from("someone02");
    println!("after modify user2 name: {:#?}", user2);

    let user3 = init_user(String::from("luojie"), String::from("someone02@111.com"));
    println!("init user data: {:#?}", user3);

    let user4 = User {
        username: String::from("罗杰"),
        ..user3 // 使用其它实例来初始化新的对象，该表示方法只能写在最后
    };
    println!("init user data from other instance: {:#?}", user4);

    // 没有名称的 struct
    // 可以使用元组解引用获取里面的值，或者使用 .索引 来读取
    struct Color(u8, u8, u8);
    struct Point(i32, i32, i32);

    let red = Color(255, 0, 0);
    println!("red is ({},{},{})", red.0, red.1, red.2);
    let origin = Point(0, 0, 0);
    println!("point is ({},{},{})", origin.0, origin.1, origin.2);
}


fn init_user(username: String, email: String) -> User {
    // 构建成员变量的时，使用同名变量
    User {
        email,
        username,
        sign_in_count: 10,
        active: true,
    }
}

