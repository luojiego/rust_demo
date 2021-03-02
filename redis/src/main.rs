extern crate redis;
use redis::Commands;

use lazy_static::lazy_static;

// static conn: redis::Connection = ; // redis::Connection{}; //::None;

lazy_static! {
    static ref client: redis::Client = {
        redis::Client::open(
            "redis://192.168.196.16:6001/"
        ).expect("connect redis err")
        // client.get_connection().expect("get connection err")
    };
}

fn main() {
    lazy_static::initialize(&client);
    println!("fecch_an_integer: {:?}", fetch_an_integer());
}

fn fetch_an_integer() -> redis::RedisResult<isize> {
    // let client = redis::Client::open(
    //     "redis://192.168.196.16:6001/"
    // ).expect("connect redis err");

    let mut conn = client.get_connection().expect("get connection err");

    let _ = conn.set("my_age", 30)?;
    
    conn.get("my_age")
}

// fn set_user_name()/* */ -> redis::RedisResult<dyn redis::FromRedisValue>{
fn set_user_name() {
    let mut conn = client.get_connection().expect("get connection err");
    conn.set("name", "luojie").expect("set user name err")
}

// fn get_user_name() -> std::result::Result<redis::Value, redis::RedisError> {
fn get_user_name() -> std::result::Result<redis::Value, redis::RedisError> {
    let mut conn = client.get_connection().expect("get connection err");
    conn.get("name")?
}