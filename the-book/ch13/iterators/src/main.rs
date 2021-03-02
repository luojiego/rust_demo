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