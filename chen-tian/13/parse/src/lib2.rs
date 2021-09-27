// pub mod lib2; 

use std::str::FromStr;
// 关联类型的 trait
use regex::Regex;
pub trait Parser {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl<T> Parser for T 
    where T: FromStr + Default,
{
    type Error = String;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        // let d = || Default::default();
        if let Some(captures) = re.captures(s) {
            captures.get(0).map_or(Err("failed to capture".to_string()), |s| {
                s.as_str().parse().map_err(|_err|"failed to parse captured string".to_string())
            })
        } else {
            Err("failed to parse string".to_string())
        }
    }

}

#[test]
fn should_work1() {
    assert_eq!(u8::parse("123abd"), Ok(123));
    assert_eq!(u8::parse("1234abd"), Err("failed to parse captured string".into()));
    // assert_eq!(u32::parse("1234abd"), 1234);
    assert_eq!(f64::parse("1234.1234abd"), Ok(1234.1234));
    assert!(f64::parse("abcd").is_err());
    // assert_eq!(f64::parse("abd"), 0f64);
}



