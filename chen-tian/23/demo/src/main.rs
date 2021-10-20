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

fn main() {
    println!("Hello, world!");
}
