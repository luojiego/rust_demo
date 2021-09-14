use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}


fn number(bus_stops:&[(i32,i32)]) -> i32 {
    bus_stops.iter().fold(0, |acc, i| acc + i.0 - i.1)
}

#[test]
fn test_number() {
    assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
    assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
    assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}

fn get_middle(s:&str) -> &str {
    let len = s.len();
    let m = (len as f32 / 2.0) as usize;
    if len % 2 == 0 {
        &s[m-1..m+1]
    } else {
        &s[m..m+1]
    }
}

#[test]
fn test_get_middle() {
    assert_eq!(get_middle("test"),"es");
    assert_eq!(get_middle("testing"),"t");
    assert_eq!(get_middle("middle"),"dd");
    assert_eq!(get_middle("A"),"A");
    assert_eq!(get_middle("of"),"of");
}

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect::<Vec<_>>()   
}

#[test]
fn test_array_diff() {
    assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
    assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
    assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
    assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
    assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
    assert_eq!(array_diff(vec![1,2,3], vec![1,2]), vec![3]);
}

fn maskify(cc: &str) -> String {
    // let len = cc.len();
    // if len <= 4 {
    //     return String::from(cc);
    // }

    // let mut v = Vec::new();
    // for _i in 0..len-4 {
    //     v.push(b'#')
    // }
    // format!("{}", String::from_utf8(v).unwrap() + &cc[len-4..])

    // 方法二
    // if cc.len() > 4 {
    //     "#".repeat(cc.len()-4) + &&cc[cc.len()-4..]
    // } else {
    //     cc.to_string()
    // }

    let mask_length = cc.len().saturating_sub(4);
    "#".repeat(mask_length) + &cc[mask_length..]
}

#[test]
fn test_maskify() {
    let x: usize = 3;
    println!("x.saturating_sub(4): {}", x.saturating_sub(4));
    assert_eq!(maskify("4556364607935616"), "############5616");
    assert_eq!(maskify("1"), "1");
    assert_eq!(maskify("11111"), "#1111");
}

#[test]
fn test_size_of() {
    let v1 = 100;
    let v2 = 100;
    
    let a = |x:i32| x;
    let b = |x:i32| x + v1;
    let c = |x:i32| x + v1 + v2;
    assert_eq!(size_of(&a), 0);
    assert_eq!(size_of(&b), 8);
    assert_eq!(size_of(&c), 16);
}

fn size_of<T>(_: &T)-> usize {
    std::mem::size_of::<T>()
}

#[test]
fn test_clouser_ownership() {
    let mut a = 1;
    let mut inc = || a += 1;
    inc();
    inc();
    println!("{}", a);
}

#[test]
fn test_clouser_ownership1() {
    let s = String::from("test");
    let f = || {
        let _s = s; println!("{}", _s);
    };
    f();
    // f(); // 编译不通过，String 未实现 Clone，闭包调用将导致 s 的所有权移除
}

#[test]
fn test_clouser_ownership2() {
    let s = String::from("test");
    let f = || {
        println!("{}", s);
    };
    f();
    f();
}

#[test]
fn test_clouser_ownership3() {
    let mut s = String::from("test");
    let mut f = || {
        s.push('a');
        println!("{}", s);
    };
    f();
    f();
}

#[test]
fn test_clouser_ownership4() {
    let s = String::from("test");
    let f = move || {
        let _s = s;
        println!("{}", _s);
        _s
    };
    f();
    // f();
}

#[test]
fn test_lifetime() {
    let mut map = HashMap::new();
    map.insert("hello", "world");
    let key = "hello1";

    match map.get_mut(&key) {
        Some(v) => do_something(v),    
        None => {
            map.insert(key, "tyr");
        }
    }
    map.insert("name", "luojie");
    println!("{:#?}", map);
}

fn do_something(_v: &mut &str) {
    todo!()
}

use itertools::Itertools;
fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    // let mut v = Vec::new();
    // for iter in words.iter() {
    //     if is_same(word, iter) {
    //         v.push(iter.clone());
    //     }
    // }
    // v
    // 依然是闭包了解的不是很好
    let cs = word.chars().sorted().collect_vec();
    words.iter().filter(|s| s.chars().sorted().collect_vec() == cs).cloned().collect()

    // words.iter().filter(|s| is_same(word,s)).cloned().collect()
    // words.into_iter().filter(|s| is_same(word, s)).cloned().collect()
}

fn _is_same(word: &str, s: &String) -> bool {
    let mut arr1 = [0u8;256];
    let mut arr2 = [0u8;256];
    for iter in word.as_bytes().iter() {
        arr1[*iter as usize] += 1;
    }

    for iter in s.as_bytes().iter() {
        arr2[*iter as usize] += 1;
    }
    if arr2 == arr1 {
        return true;
    }
    false
}

#[test]
fn sample_tests() {
    let mut _s = "test".to_string();
    // println!("{}", s);
    // let s1 = s;
    // println!("{}", s1);
    // let s2 = s;
    // println!("{}", s2);
    // let s = s.as_mut_bytes().sort();
    // unsafe {
    //     s.as_bytes_mut().sort();
    // }
    // println!("{}", s);
    // assert_eq!(s, "estt".to_string());
    
    do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);
    
    do_test(
        "racer",
        &["crazer", "carer", "racar", "caers", "racer"],
        &["carer", "racer"],
    );
}

fn do_test(word: &str, words: &[&str], exp: &[&str]) {
    let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
    let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
    let got = anagrams(word, &words);
    assert_eq!(
        got, expected,
        "Failed with word: \"{}\"\nwords: {:#?}",
        word, words
    );
}

#[test]
fn test_bubble_sort() {
    let mut v = vec![8, 2, 9, 1, 105, 3, 10, 6];
    bubble_sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 6, 8, 9, 10, 105])
}

fn bubble_sort(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        for j in 0..v.len()-i-1 {
            if v[j] > v[j+1] {
                // let tmp = v[j];
                // v[j] = v[j+1];
                // v[j+1] = tmp;
                v.swap(j, j+1);
            }
        }
    }
}