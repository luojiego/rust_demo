use std::{marker::PhantomData, sync::atomic::{AtomicU64, Ordering}};

#[warn(dead_code)]
pub struct BufReader<R> {
    inner: R,
    buf: Box<[u8]>,
    pos: usize,
    cap: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Identifier<T> {
    inner: u64,
    _tag: std::marker::PhantomData<T>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    id : Identifier<Self>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Product {
    id : Identifier<Self>,
}

#[test]
fn id_should_be_the_same() {
    let user = User::default();
    let product = Product::default();
    // 这行编译器都不让通过
    // assert_eq!(user.id, product.id);
    assert_eq!(user.id.inner, product.id.inner);
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
pub struct Customer<T> {
    id: u64, 
    name: String, 
    _type: PhantomData<T>,
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) { 
        println!(
            "Dear {} (as our valuable customer {}), enjoy this advanced feature!", 
            self.name, self.id
        )}
}

impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name, 
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    customer.into()
}

pub struct FreePlan;
pub struct PersonalPlan(f32);

#[test]
fn test_customer() {
    let customer = Customer::<FreePlan>::new("Tyr".into());

    customer.feature1();
    customer.feature2();

    let customer = subscribe(customer, 6.99);
    customer.feature1();
    customer.feature2();

    customer.advance_feature();
}

#[derive(Debug, Default)]
pub struct Equation<IterMethod> {
    current: u32,
    _method: PhantomData<IterMethod>,
}

// 线性增长
#[derive(Debug, Default)]
pub struct Linear;

// 二次增长
#[derive(Debug, Default)]
pub struct Quadratic;

impl Iterator for Equation<Linear> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None;
        }
        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u16::MAX as u32 {
            return None;
        }
        Some(self.current * self.current)

    }
}

#[test]
fn test_linear() {
    let mut equation = Equation::<Linear>::default();
    assert_eq!(Some(1), equation.next());
    assert_eq!(Some(2), equation.next());
    assert_eq!(Some(3), equation.next());
}

#[test]
fn test_quadratic() {
    println!("u16::MAX {}", u16::MAX);
    println!("u16::MAX^2 {}", u16::MAX as u32 * u16::MAX as u32);
    println!("u32::MAX {}", u32::MAX);
    let mut equation = Equation::<Quadratic>::default();
    assert_eq!(Some(1), equation.next());
    assert_eq!(Some(4), equation.next());
    assert_eq!(Some(9), equation.next());
}

pub fn consume_iterator<F, Iter, T>(mut f: F)
where
    F: FnMut(i32) -> Iter,
    Iter: Iterator<Item = T>, 
    T: std::fmt::Debug,
{
    for item in f(10) {
        println!("{:?}", item);
    }
}

#[test]
fn test_consume_iterator() {
    consume_iterator(|i| (0..i).into_iter());
}
fn main() {
    println!("Hello, world!");
}
