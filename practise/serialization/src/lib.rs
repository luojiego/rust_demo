use std::io::{Error, Write};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Unspecified = 0,
    Mail = 1,
    Femails = 2,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Unspecified
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    age: u8, // 调在当前 module 可以调用
    pub(crate) gender: Gender, // 仅在此 crate 能调用
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self { 
        Self { 
            name, age, gender 
        } 
    }

    pub fn serialization(&self, path: &str) -> Result<usize, Error> {
        let mut file = File::create(path)?;
        let data = serde_json::to_string(&self).unwrap();
        let size = file.write(data.as_bytes())?;
        Ok(size)
    }

    pub fn deserialize_from_file(path: &str) -> Result<Self, Error> {
        let mut data = String::new();
        File::open(path)?.read_to_string(&mut data)?;

        Ok(serde_json::from_str(data.as_str()).unwrap())
    }
}

// 默认实现
impl Default for User {
    fn default() -> Self {
        Self { 
            name: "unknown name".into(), 
            age: Default::default(), 
            gender: Default::default() 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let path = "user.json";
        let user = User::default();
        println!("default: {:?}", user);
        user.serialization("./user.json").unwrap();
        let user1 = User::deserialize_from_file(path).unwrap();
        assert_eq!(user, user1);
    }
}
