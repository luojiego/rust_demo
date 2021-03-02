fn main() {
    {
        let mut s = String::new();
        println!("s = {}", s);
    }
    {
        let data = "initial contents";
        let s1 = data.to_string();
        println!("s1 = {}", s1);
        let s2 = "initial contents".to_string();
        println!("s2 = {}", s2);
        let s3 = String::from("initial contents");
        println!("s3 = {}", s3);
    }
    {
        let hello = String::from("السلام عليكم");
        println!("{}", hello);
        let hello = String::from("Dobrý den");
        println!("{}", hello);
        let hello = String::from("Hello");
        println!("{}", hello);
        let hello = String::from("שָׁלוֹם");
        println!("{}", hello);
        let hello = String::from("नमस्ते");
        println!("{}", hello);
        let hello = String::from("こんにちは");
        println!("{}", hello);
        let hello = String::from("안녕하세요");
        println!("{}", hello);
        let hello = String::from("你好");
        println!("{}", hello);
        let hello = String::from("Olá");
        println!("{}", hello);
        let hello = String::from("Здравствуйте");
        println!("{}", hello);
        let hello = String::from("Hola");
        println!("{}", hello);
    }
    {
        // no method named `push_str` found for reference `&str` in the current scope
        // let mut s = "luo";
        let mut s = String::from("luo");
        s.push_str(" jie");
        println!("s = {}", s);
    }
    {
        let mut s = String::from("Roge");
        s.push('r');
        println!("s = {}", s);
    }
    {
        // a + b
        let s1 = String::from("luo ");
        let s2 = String::from("jie");
        // let name = s1 + s2; // expected `&str`, found struct `String`
        // let name = &s1 + s2; // `+` cannot be used to concatenate a `&str` with a `String`
        let name = s1 + &s2;
        println!("name = {}", name);
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("s = {}", s);
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3); 
        println!("format = {}", s);
    }
}
