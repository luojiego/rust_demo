fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<usize>();

    is_copy::<fn()>();

    is_copy::<*const String>();
    is_copy::<*mut String>();

    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    is_copy::<[u8;4]>();
    is_copy::<(&str,&str)>();
}

// 以下类型均没有实现 Copy
fn types_not_impl_copy_trait() {
    // is_copy::<str>();
    // is_copy::<[u8]>();
    // is_copy::<Vec[u8]>();
    // is_copy::<String>();
    // is_copy::<&mut String>();
    // is_copy::<[Vec<u8>;4]>();
    // is_copy::<(String, u32)>();
}

fn main() {
    types_impl_copy_trait();
    types_not_impl_copy_trait();

    let data = vec![10, 42, 9, 8];
    let v = 42;
    if let Some(pos) = find_pos(data, v) {
        println!("Found {} at {}", v, pos);
    }
}

fn find_pos(data: Vec<u32>, v:u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }
    None
}
