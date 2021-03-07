fn give_me<T> (value: T) {
    let _ = value;
}

// cargo build
// nm target/debug/generic | grep give_me

fn main() {
    let a = "generics";
    let b = 10;

    give_me(a);
    give_me(b);
}
