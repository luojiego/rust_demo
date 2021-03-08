fn main() {
    // let a = String::from("Owned string");
    // let a_ref = &mut a;
    // //          ^^^^^^ cannot borrow as mutable 
    // a_ref.push('!');

    let mut a = String::from("Owned string");
    let a_ref = &mut a;
    a_ref.push('!');
    println!("{}", a);
}