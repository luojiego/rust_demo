struct Game;
struct Enemy;
struct Hero;

trait Loadable {
    fn init(&self);
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("Enemy loaded");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("Hero loaded");
    }
}

impl Game {
    fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}

use std::{fmt::Display, ops::Add};

fn add_something<T:Add>(fst: T, snd: T) {
    let _ = fst + snd;
}

trait Eatable {
    fn eat(&self);
}

#[derive(Debug)]
struct Food<T>(T);

#[derive(Debug)]
struct Apple;

use std::fmt::Debug;

impl<T> Eatable for Food<T> where T: Debug {
    fn eat(&self) {
        println!("Eating {:?}", self);
    }
}

fn eat<T>(val: T) where T: Eatable {
    val.eat();
}

trait Eat {
    fn eat(&self) {
        println!("eat");
    }
}

trait Code {
    fn code(&self) {
        println!("code");
    }
}

trait Sleep {
    fn sleep(&self) {
        println!("sleep");
    }
}

trait Programmer: Eat + Code + Sleep {
    fn animate(&self) {
        self.eat();
        self.code();
        self.sleep();
        println!("repeat!");
    }
}

struct Bob;

impl Programmer for Bob {}
impl Eat for Bob {}
impl Code for Bob {}
impl Sleep for Bob {}

fn show_me(val: impl Display) {
    println!("{}", val);
}

fn lazy_adder(a: u32, b: u32) -> impl Fn() -> u32 {
    move || a + b
}

fn surround_with_braces(val: impl Display) -> impl Display {
    format!("{{{}}}", val)
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);

    add_something(2, 2);
    // add_something("hello".to_string(), "world".to_string());
    add_something(2.3, 2.5);
    add_something(2 as f32, 2.5);

    let apple = Food(Apple);
    eat(apple);

    Bob.animate();

    show_me("Trait bounds are awesome");
    show_me(3.5);
    show_me(70_000_123);

    let add_later = lazy_adder(1024, 2048);
    println!("{:?}", add_later());

    println!("{}", surround_with_braces("hello"));
}
