
pub mod abi;


use abi::{command_request::RequestData, *};
use anyhow::{Result, anyhow};
use bytes::Bytes;
use http::StatusCode;
use prost::Message;

use crate::KvError;

impl CommandRequest {
    /// 创建 HSET 命令
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall{
                table: table.into(),
            })),
        }
    }

    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self { 
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }

    pub fn new_hdel(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self { 
            request_data: Some(RequestData::Hdel(Hdel {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hexists(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self { 
            request_data: Some(RequestData::Hexist(Hexist {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hmset(table: impl Into<String>, pairs: impl Into<Vec<Kvpair>>) -> Result<Self> {
        let p = pairs.into().clone();
        let p1 = p.clone();

        for i in p.iter() {
            if let Some(_) = &i.value {
            } else {
                return Err(anyhow!("value is None"));
            }
        }

        Ok(Self {
            request_data: Some(RequestData::Hmset(Hmset {
                table: table.into(),
                pairs: p1,
            }))
        })
    }

    pub fn new_hmget(table: impl Into<String>, keys: impl Into<Vec<String>>) -> Self {
        Self {
            request_data: Some(RequestData::Hmget(Hmget {
                table: table.into(),
                keys: keys.into(),
            }))
        }
    }

    pub fn new_hmexsits(table: impl Into<String>, keys: impl Into<Vec<String>>) -> Self {
        Self {
            request_data: Some(RequestData::Hmexist(Hmexist {
                table: table.into(),
                keys: keys.into(),
            }))
        }
    }
}

impl Kvpair {
    /// 创建一个新的 kv pair
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// 从 String 转换成 Value
impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

/// 从 &str 转换成 Value
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(i)),
        }
    }
}

impl From<bool> for Value {
    fn from(i: bool) -> Self {
        Self {
            value: Some(value::Value::Bool(i)),
        }
    }
}

impl From<bool> for CommandResponse {
    fn from(i: bool) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![i.into()],
            ..Default::default()
        }
    }
}

impl From<String> for CommandResponse {
    fn from(s: String) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![s.into()],
            ..Default::default()
        }
    }
}

impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(v: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: v,
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        };

        match e {
            KvError::NotFound(_,_) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }
        result
    }
}

impl From<Option<Value>> for CommandResponse {
    fn from(e: Option<Value>) -> Self {
        if let Some(v) = e {
            Self {
                status: StatusCode::OK.as_u16() as _,
                values: vec![v],
                ..Default::default()
            }
        } else {
            Self {
                status: StatusCode::OK.as_u16() as _,
                values: vec![Value::default()],
                ..Default::default()
            }
        }

    }
}

impl From<(String, Value)> for Kvpair {
    fn from(data: (String, Value)) -> Self {
        Kvpair::new(data.0, data.1)
    }
}

impl TryFrom<Value> for i64 {
    type Error = KvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Integer(i)) => Ok(i),
            _ => Err(KvError::ConvertError(v, "Integer")),
        }
    }
}

impl TryFrom<Value> for Bytes {
    type Error = KvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Binary(b)) => Ok(b),
            _ => Err(KvError::ConvertError(v, "Binary")),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = KvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Bool(b)) => Ok(b),
            _ => Err(KvError::ConvertError(v, "Boolean"))
        }
    }
}


impl TryFrom<Value> for Vec<u8> {
    type Error = KvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let mut buf = Vec::with_capacity(v.encoded_len());
        v.encode(&mut buf)?;
        Ok(buf)
    }
}

impl TryFrom<&[u8]> for Value {
    type Error = KvError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let msg = Value::decode(data)?;
        Ok(msg)
    }
}