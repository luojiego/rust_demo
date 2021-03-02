extern crate protobuf;
// use protobuf_demo::Person;
// cargo install protobuf-codegen 安装相关软件包 protoc-gen-rust
pub mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}

fn main() {
    println!("Hello, world!");
    // let p = crate::Person::new();
    // println!("p: {}", p);
}

