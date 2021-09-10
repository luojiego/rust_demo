fn main() {
    let data = vec![1,2,3,4,5,6,7,8,9];
    let data1 = &data;
    println!("addr of value: {:p} {:p}, addr of data: {:p} {:p}", &data, data1, &&data, &data1);

    println!("sum of data1: {}", sum(data1));

    // 堆上的地址是什么
    println!("addr of items: [{:p}, {:p}, {:p}, {:p}]", &data[0], &data[1], &data[2], &data[3]);
}

fn sum(v: &Vec<u32>) -> u32 {
    println!("addr of value: {:p}, addr of ref: {:p}", v, &v);
    v.iter().fold(0, |acc,x| acc + x)

}

#[test]
fn test_push_2_vec_when_iterator() {
    let mut v = vec![1,2,3];
    // for item in v.iter() {
    //     v.push(*item)
    // }
    v.push(2);
    v.push(3);
    v.push(4);
//  ------------
//  |                 |
//  |                 first mutable borrow occurs here
//  |                 first borrow later used here
//  |      v.push(*item+1)
//  |      ^ second mutable borrow occurs here
}

#[test]
fn test1() {
    let mut arr = vec![1, 2, 3];  
    // cache the last item  
    let last = arr.last();  
    // consume previously stored last item  
    println!("last: {:?}", last);
    arr.push(4);  
}

fn test2() {
    let mut arr = vec![1, 2, 3];  
    // cache the last item  
    let last = arr[arr.len() - 1];   
    // consume previously stored last item  
    println!("last: {:?}", last);
    arr.push(4);  
}

#[test]
fn test_ownership() {
    // Rust 仅允许有一个活跃的可变引用
    let mut a = 3;
    let b = &mut a;
    *b = 4;
    println!("after mut b(&a) modify a = {}", a);
    let c = &mut a;
    *c = 5;
    println!("after mut c(&a) modify a = {}", a);
    let d = &mut a;
    *d = 6;
    println!("after mut d(&a) modify a = {}", a);
}

#[test]
fn test_mem() {

    let mut v = vec![1];
    let v1: Vec<i32> = Vec::with_capacity(8);
    print_vec("v1", v1);

    println!("heap start: {:p}", &v[0] as *const i32);
    extend_vec(&mut v);

    println!("new heap start: {:p}", &v[0] as *const i32);

    print_vec("v", v);
}

fn extend_vec(v: &mut Vec<i32>) {
    (2..34).into_iter().for_each(|i| v.push(i));
}

fn print_vec<T>(name: &str, data: Vec<T>) {
    use std::mem;
    let p: [usize; 3] = unsafe {mem::transmute(data)};
    println!("{}: 0x{:x}, {}, {}", name, p[0], p[1], p[2])
}
