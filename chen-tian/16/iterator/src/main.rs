use itertools::Itertools;
fn main() {
    {
        let result = vec![1,2,3,4]
            .iter()
            .map(|v| v * v)
            .filter(|v| *v < 16)
            .skip(1)
            .collect::<Vec<_>>();
            
        println!("{:?}", result);
    }
    {
        let err_str = "bad happend";
        let input = vec![Ok(21), Err(err_str), Ok(7)];
        let it = input
            .into_iter()
            .filter_map_ok(|i| if i>10 {Some(i*2)} else {None});
        println!("{:?}", it.collect::<Vec<_>>());
    }
    let mut v1 = vec![1,2,3,4];
    v1.push(5);
    println!("cap should be 8: {:?}", v1.capacity());

    let b1 = v1.into_boxed_slice();
    let mut b2 = b1.clone();

    let v2 = b1.into_vec();
    println!("cap should be exactly 5: {}", v2.capacity());

    use std::ops::Deref;
    assert!(b2.deref() == v2);

    b2[0] = 2;
    println!("b2: {:?}", b2);

    let b3 = Box::new([2,2,3,4,5]);
    println!("b3: {:?}", b3);

    assert!(b2 == b3);
}
