use crate::{KvError, Kvpair, Value};
mod memory;
pub use memory::Memtable;

mod sleddb;
pub use sleddb::SledDb;

pub trait Storage {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

/// 提供 Storage iterator，这样 trait 的实现者只需要把它们的 iterator 提供
/// 给 StorageIter，然后它们保证 next() 传出的类型实现了 Into<Kvpair> 即可
pub struct StorageIter<T> {
    data: T,
}

impl<T> StorageIter<T> {
    pub fn new(data: T) -> Self {
        Self  { data }
    }
}

impl<T> Iterator for StorageIter<T> 
where
    T: Iterator,
    T::Item: Into<Kvpair>,
{
    type Item = Kvpair;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v|v.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = Memtable::new();
        test_basic_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = Memtable::new();
        test_get_all(store);
    }

    fn test_basic_interface(store: impl Storage) {
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());

        let v1 = store.set("t1", "hello".into(), "world1".into());
        assert_eq!(v1, Ok(Some("world".into())));

        let v = store.get("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        assert_eq!(Ok(None), store.get("t1", "hello1"));
        assert!(store.get("t2", "hello1").unwrap().is_none());

        assert_eq!(store.contains("t1", "hello"), Ok(true));
        assert_eq!(store.contains("t1", "hello1"), Ok(false));
        assert_eq!(store.contains("t2", "hello"), Ok(false));

        let v = store.del("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        assert_eq!(Ok(None), store.del("t1", "hello1"));
        assert_eq!(Ok(None), store.del("t2", "hello"));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();
        let mut data = store.get_all("t2").unwrap();

        data.sort_by(|a, b|a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into()),
            ]
        )
    }

    fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap(); 
        let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(            
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())            
            ]        
        )
    }

    #[test]
    fn memtable_iter_should_work() {
        let store = Memtable::new();
        test_get_iter(store);
    }
}