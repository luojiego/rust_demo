use crate::*;
use Storage;
use http::StatusCode;
impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => Value::default().into(),
        }
    }
}

impl CommandService for Hdel {
    fn execute(self, store: &impl Storage) -> CommandResponse { 
        match store.del(&self.table, &self.key) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hexist{
    fn execute(self, store: &impl Storage) -> CommandResponse { 
        match store.contains(&self.table, &self.key) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

// 需要遍历写入数据
impl CommandService for Hmset {
    fn execute(self, store: &impl Storage) -> CommandResponse { 
        for i in self.pairs.iter() {
            if let Some(value) = i.clone().value {
                match store.set(&self.table, i.key.clone(), value) {
                    Ok(_) => {
                    },
                    Err(e) => {
                        return e.into();
                    }
                }
            }
      }
      "Ok".to_string().into()
    }
}

impl CommandService for Hmget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let mut res = CommandResponse{
            status: StatusCode::OK.as_u16() as _,
            values: vec![],
            ..Default::default()
        };
        for i in self.keys.iter() {
            match store.get(&self.table, &i) {
                Ok(Some(v)) => res.values.push(v),
                Ok(None) => {
                    return KvError::NotFound(self.table, i.clone()).into();
                },
                Err(e) => {
                    return e.into();
                }
            };
        }
        res
    }
}

impl CommandService for Hmexist {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let mut res = CommandResponse{
            status: StatusCode::OK.as_u16() as _,
            values: vec![],
            ..Default::default()
        };
        for i in self.keys.iter() {
            match store.contains(&self.table, &i) {
                Ok(v) => res.values.push(v.into()),
                Err(_) => {
                }
            };
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hset_should_work() {
        let store = Memtable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["world".into()], &[]);
    }

    #[test]
    fn new_hmset_should_error() {
        let cmd = CommandRequest::new_hmset("t1", 
        vec![
                Kvpair { key: "name".into(), value: Some("LuoJie".to_string().into()) }, 
                Kvpair { key: "age".into(), value: None },
        ]);
        if let Err(e) = cmd {
            println!("error occurs: {}", e);
        } else {
            assert!(1==2);
        }
    }

    #[test]
    fn hmset_should_work() {
        let store = Memtable::new();
        let cmd = CommandRequest::new_hmset("t1", 
        vec![
                Kvpair { key: "name".into(), value: Some("LuoJie".to_string().into()) }, 
                Kvpair { key: "age".into(), value: Some(31.into()) },
                Kvpair { key: "address".into(), value: Some("Xi'an".to_string().into()) },
        ]);
        let res = dispatch(cmd.unwrap(), &store);
        // let v = cmd.unwrap();
        assert_eq!(res.values[0], "Ok".into());
        let cmd = CommandRequest::new_hmget("t1", vec!["name".into(), "age".into()]);
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &["LuoJie".into(), 31.into()], &[]);
        let cmd = CommandRequest::new_hmget("t1", vec!["name".into(), "address".into()]);
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &["LuoJie".into(), "Xi'an".into()], &[]);
    }

    fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
        res.pairs.sort_by(|a,b|a.partial_cmp(b).unwrap());
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "");
        assert_eq!(res.values, values);
        assert_eq!(res.pairs, pairs);
    }
}