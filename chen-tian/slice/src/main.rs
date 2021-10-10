use std::fmt;
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    let s1 = &arr[..2];
    let s2 = &vec[..2];
    println!("s1: {:?}, s2: {:?}", s1, s2);

    assert_eq!(s1, s2);

    assert_eq!(&arr[..], vec);
    assert_eq!(&vec[..], arr);

    let v = vec![1, 2, 3, 4];

    print_slice(&v);
    print_slice(&v[..]);

    print_slice(&v);
    print_slice(&v[..]);
    print_slice1(v);

    let arr = [1, 2, 3, 4, 5];
    print_slice(&arr);
    print_slice(&arr[..]);
    print_slice1(&arr);
    print_slice1(&arr[..]);
    print_slice1(arr);
}

fn print_slice<T: fmt::Debug>(s: &[T]) {
    println!("{:?}", s);
}

fn print_slice1<T, U>(s: T)
where 
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}
