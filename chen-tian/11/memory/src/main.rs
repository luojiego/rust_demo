use std::{collections::HashMap, mem::{size_of, align_of}};

struct S1 {
    a: u8, 
    b: u16,
    c: u8,
}

struct S2 {
    a: u8, 
    b: u8,
    c: u16,
}

enum E {
    A(f64),
    B(HashMap<String,String>),
    C(Result<Vec<u8>, String>),
}

macro_rules! show_size {
    (header) => {
        println!("{:<24} {:>4}      {}      {}",
        "Type", "T", "Option<T>", "Result<T, io::Error>");
        println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!("{:<24} {:>4} {:8} {:12}",
        stringify!($t),
        size_of::<$t>(),
        size_of::<Option<$t>>(),
        size_of::<Result<$t, std::io::Error>>(),
        )
    };
}

#[derive(Debug)]
struct Packet<const N:usize> {
    data: [u8; N],
}

fn match_range(v: usize) -> &'static str {
    match v {
        0..=99 => "good",
        100..=9999 => "unbelievable",
        10000.. => "beyond expectation",
        _ => unbelievable!(),
    }
}

fn main() {
    println!("{}", match_range(100001));
    let ip = Packet{data: [0u8; 20]};
    let udp = Packet{data: [0u8; 8]};
    println!("ip: {:?}, udp: {:?}", ip, udp);
    // println!("sizeof S1: {}, sizeof S2: {},", size_of::<S1>(), size_of::<S2>());
    // println!("alignof S1: {}, alignof S2: {},", align_of::<S1>(), align_of::<S2>());

    // println!("sizeof Result<String,()>: {}", size_of::<Result<String, ()>>());

    println!("{}", "-".repeat(64));
    show_size!(header);
    show_size!(u8);
    show_size!(u64);
    show_size!(&u8);
    show_size!(Box<u8>);
    show_size!(&[u8]);
    
    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);
}
