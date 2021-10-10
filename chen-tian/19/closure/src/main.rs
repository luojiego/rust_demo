use std::{collections::HashMap, mem::size_of_val};

fn view_closure_size() {
    let c1 = || println!("hello world");
    let c2 = |i: i32| println!("hello {}", i);

    let name = String::from("tyr");
    let name1 = name.clone();

    let mut table = HashMap::new();
    table.insert("hello", "world");

    let c3 = || println!("hello: {}", name);
    let c4 = move || println!("hello: {}, {:?}", name1, table);

    let name2 = name.clone();
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsy");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    )
}

fn closure_fn_once() {
    let name = String::from("Tyr");
    let c = move |greeting: String| (greeting, name); 
    let result = c("hello".into());
    println!("result: {:?}", result);

    // 编译器报错：use of moved value: `c` value used here after move
    // let result = c("hi".into());
}

fn not_closure_fn_once() {
    let name = String::from("Tyr");
    let c = move |greeting: String| (greeting, name.clone()); 
    println!("c1 call once: {:?}", c("qiao".into()));
    println!("c1 call twice: {:?}", c("bonjour".into()));

    println!("result: {:?}", call_once("hi".into(), c));

    // borrow of moved value: `c` value borrowed here after move
    // let result = c("hi".to_string());

    println!("result: {:?}", call_once("hola".into(), not_closure));
}

fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg) 
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}

fn fn_mut() {
    let mut name = String::from("hello");
    let mut name1 = String::from("hola");

    // name 被闭包作为可变引用传入了
    let mut c = || {
        name.push_str(" Tyr");
        println!("c: {}", name);
    };

    // 因为使用了 move，所有在此闭包之后，name1 便不能再被访问
    let mut c1 = move || {
        name1.push_str("!");
        println!("c1: {}", name1);
    };

    c();
    c1();

    call_mut(&mut c);
    call_mut(&mut c1);

    call_once1(c);
    call_once1(c1);

    name.push_str(" &&");
    println!("name: {}", name);
    // println!("name1: {}", name1);

    // call_mut(&mut c);

    // use of moved value: `c` value used here after move
    // call_once1(c);
}

fn call_mut(c: &mut impl FnMut()) {
    c();
}

fn call_once1(c: impl FnOnce()) {
    c();
}

fn call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
    c(arg)
}

fn call_mut1(arg: u64, c: &mut impl FnMut(u64) -> u64) ->u64 {
    c(arg)
}

fn call_once2(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}

fn fn_learn() {
    let v = vec![0u8; 1024];
    let v1 = vec![0u8; 1023];

    let mut c = |x : u64| v.len() as u64 * x;
    let mut c1 = move|x: u64| v1.len() as u64 * x;

    println!("direct call c(2): {}", c(2));
    println!("direct call c1(2): {}", c1(2));

    println!("call call(3,&c): {}", call(3, &c));
    println!("call call(3,&c1): {}", call(3, &c1));

    println!("call_mut: {}", call_mut1(4, &mut c));
    println!("call_mut: {}", call_mut1(4, &mut c));

    println!("call_once: {}", call_once2(5, c));
    println!("call_once: {}", call_once2(5, c1));
}

fn main() {
    closure_fn_once();
    not_closure_fn_once();
    fn_mut();
    fn_learn();
}
