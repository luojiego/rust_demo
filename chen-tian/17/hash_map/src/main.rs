use std::collections::HashMap;

fn hash_map_info() {
    let mut map = HashMap::new();
    explain("empty", &map);

    map.insert('a', 1);
    explain("insert <'a', 1>", &map);
    map.insert('b', 2);
    map.insert('c', 3);
    explain("insert <'b', 2>, <'c', 3>", &map);

    map.insert('d', 4);
    explain("insert <'d',4>", &map);

    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

    map.remove(&'a');
    assert_eq!(map.contains_key(&'a'), false);
    assert_eq!(map.get(&'a'), None);
    explain("removed", &map);
    
    map.shrink_to_fit();
    explain("shrinked", &map);
} 

fn vector_info() {
    let mut v = Vec::new();
    explain_vec("empty", &v);
    v.push(1);
    explain_vec("after push 1", &v);
    v.push(2);
    v.push(3);
    explain_vec("after push 2 and 3", &v);
    v.push(4);
    explain_vec("after push 4", &v);
    v.push(5);
    explain_vec("after push 5", &v);

    v.pop();
    println!("{:?}", v);
    explain_vec("after pop", &v);

    v.shrink_to_fit();
    explain_vec("after shrink", &v);
}

fn main() {
    hash_map_info();
    println!("-------------------------------------");
    vector_info();
    vecotr_cap_change();
}

fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}: len: {}, cap: {}", name, map.len(), map.capacity());
}

fn explain_vec<T>(desc: &str, v: &Vec<T>) {
    println!("{}: len: {}, cap: {}", desc, v.len(), v.capacity());
}

fn vecotr_cap_change() {
    let mut v = Vec::new();
    let mut cap = v.capacity();
    let mut len = v.len();
    println!("len: {}, cap: {}", len, cap);
    for i in 0..100000 {
        v.push(i);
        if v.capacity() != cap {
            println!("cap change len: {}, cap: {}", v.len(), v.capacity());
            len = v.len();
            cap = v.capacity();
        }
    }
}