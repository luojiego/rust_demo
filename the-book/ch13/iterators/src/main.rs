use std::collections::HashMap;

// use std::iter::Filter;

fn main() {
    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    let x = 3;
    let r = is_odd(3);
    println!("is_odd({}) = {}", x, r);
}

fn is_odd(i: i32) -> bool {
    i % 2 == 0
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: u32 = v1_iter.sum();
    
    // 也可以指定为 i32
    // let total: i32 = v1_iter.sum();
    // 编译报错，必须提供类型
    // let total = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_produce() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x|x+1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq,Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s|s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            }
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![ 
                Shoe{
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe{
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a,b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    let iter = Counter::new().zip(Counter::new().skip(1));
    for val in iter {
        println!("zip Got: {:?}", val);
    }

    let v = vec![1, 2, 3, 4, 5, 6];
    let iter = Counter::new().zip(v.iter().skip(3)).map(|(a,b)| a * b);
    for val in iter {
        println!("\tzip Got: {:?}", val);
    }

    let iter = Counter::new().zip(Counter::new().skip(1)).map(|(a,b)| a * b);
    for val in iter {
        println!("map Got: {:?}", val);
    }

    let iter = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a,b)| a * b)
        .filter(|x| x % 3 == 0);
    for val in iter {
        println!("filter Got: {:?}", val);
    }
    // !!! 重要
    // 不能使用 iter 的中间值
    assert_eq!(18, sum);
}

#[test]
fn test_iter_skip() {
    let v = vec![1,2,3,4,5,6];
    let v = v.iter().skip(3);
    for iter in v {
        println!("Got {}", iter);
    }

    // let v: Vec<_> = v.iter().skip(3).collect();
    // println!("{:#?}", v)
}

#[test]
fn test_iter_map() {
    let v = vec![1,2,3,4,5,6];
    let v: Vec<_> = v.iter().map(|x| x * 100).collect();
    assert_eq!(v, vec![100,200,300,400,500,600])
}

#[test]
fn test_iter_filter() {
    let v = vec![1,2,3,4,5,6]; // 3 6 9 12 15 18
    let v: Vec<_> = v.iter().map(|x|x * 3).filter(|x|x % 2 == 0).collect();
    assert_eq!(v, vec![6, 12, 18]);

    let v: Vec<_> = (0..10).filter(|x|x % 3 == 0).collect();
    assert_eq!(v, vec![0, 3, 6, 9]);

    let v = vec![1,2,3];
    let v: Vec<_> = v.iter().filter(|&&x|x%2==0).collect();
    println!("{:#?}", v);
    assert_eq!(v, vec![&2]);

    let v1 = vec![1,2,3,4,5,6];
    let v2 = vec![1,2,3,4,5,6];
   
    let v: HashMap<_, _> = v1.iter().zip(v2.iter().skip(3)).collect();
    println!("{:#?}", v);

    let sum = v2.iter().fold(0, |acc,x| acc + x);
    println!("sum: {}", sum);

    let a = [0i32, 1, 2];

    let mut iter = a.iter().filter(|x| x.is_positive());

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_fold() {
    let v = vec![1,2,3,4,5,6];
    let sum = v.iter().fold(0,|acc,x|acc+x);
    assert_eq!(sum, 21);
    assert_eq!(v.iter().sum::<i32>(), 21);

    let v = v.iter().map(|x| x*x).collect::<Vec<_>>();
    assert_eq!(v, vec![1, 4, 9, 16, 25, 36]);
}

#[test]
fn test_iter_rev() {
    let v = vec![1,2,3,4,5,6];
    let v = v.iter().rev().collect::<Vec<_>>();
    assert_eq!(v, vec![&6,&5,&4,&3,&2,&1])
}

#[test]
fn test_iter_chain() {
    let v = (1..3).chain(8..10).collect::<Vec<_>>();
    assert_eq!(v, vec![1,2,8,9])
}

#[test]
fn test_iter_sum() {
    let v = vec![1, 2, 4, 8, 16, 32, 64];
    let sum: i64 = v.iter().sum();
    assert_eq!(sum as f64, 127.0);
    assert_eq!(v.iter().max(), Some(&64));
    assert_eq!(v.iter().min(), Some(&1));
}