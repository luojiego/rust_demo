// pub fn strtok<'b, 'a>(s :&'b mut &'a str, delimiter: char) -> &'a str {
// pub fn strtok<'a>(s :&mut &'a str, delimiter: char) -> &'a str {
pub fn strtok<'a>(s :&mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i+delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}

#[test]
fn test_string_2_str() {
    let s = String::from("hello world");
    let str = s.as_str();
    let s1: &str = s.as_ref();
    assert_eq!(str, s1);
}

#[test]
fn test_str_2_string() {
    let str = "hello world";
    let s = "hello world".to_string();
    assert_eq!(s, String::from(str));
    let s: String = "hello world".into();
    assert_eq!(s, String::from(str));
    let s = "hello world".to_owned();
    assert_eq!(s, String::from(str));
}

#[test]
fn test_integer_convert() {
    let a = 1;
    let b = 0;
    let c = -3.2f32;
    let d = 3.0;
    let n = c.sqrt();
    println!("n: {}", n);
    let e = d as i32;
    println!("e: {}", e);

    // let a = -32f32;
    // let b = -1.0f32;
    // println!("{}", a.sqrt());
    // assert_eq!(a.sqrt(), b.sqrt());
    println!("i32::MAX: {} u32::MAX: {}", i32::MAX, u32::MAX);
    println!("i64::MAX: {} u64::MAX: {}", i64::MAX, u64::MAX);
    println!("f64::MAX: {:10.3e}", f64::MAX);
}

#[test]
fn test_float() {
    let (a, b, c) = (0.1f32, 0.2f32, 0.3f32);
    let (x, y, z) = (0.1f64, 0.2f64, 0.3f64);
    assert_eq!(a + b, c);
    assert_ne!(x + y, z);
    assert!(x + y - z < f64::EPSILON);
    println!("a + b: {}", a + b);
    println!("x + y: {}", x + y);
    println!("{:?}", f32::EPSILON);
    println!("{:?}", f64::EPSILON);
}

fn test<T>(x: T) where T: Fn(){
    x()
}

// fn test_mut(x: T) {
//     x();
// }

#[test]
fn test_clouser_arg() {
    let s = String::from("clouser args");
    let x = ||println!("{}", s);
    test(x);
}

fn test_mut<T>(mut x: T) where T: FnMut() {
    x();
}

#[test]
fn test_clouser_mut_arg() {
    let mut s = String::from("clouser mut args");
    let f = ||{
        s.push('.');
        println!("{}", s);
    };
    test_mut(f);
}

fn test_once<T>(mut f: T) where T: FnOnce() {
    f();
}

#[test]
fn test_clouser_once() {
    let s = String::from("clouser once args");
    let f = || {
        let _s = s;
    };
    test_once(f);
}
