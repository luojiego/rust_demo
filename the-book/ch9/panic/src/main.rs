fn main() {
    // {
    //     panic!("crash and burn");
    // }
    
    {
        let v = vec![1,2,3];
        println!("v.get(99): {:?}", v.get(99));
        println!("v.get(3): {:?}", v.get(2));
        v[99];
    }
}
