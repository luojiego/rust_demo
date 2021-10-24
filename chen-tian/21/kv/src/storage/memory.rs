use crate::{KvError, Kvpair, Storage, Value};
use dashmap::{mapref::one::Ref, DashMap};

#[derive(Clone, Debug, Default)]
pub struct Memtable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl Memtable {
    pub fn new() -> Self {
        Self::default()
    }    

    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for Memtable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        todo!()
    }

    fn set(&self, table: &str, key: &str, value: Value) -> Result<Option<Value>, KvError> {
        todo!()
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        todo!()
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        todo!()
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        todo!()
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        todo!()
    }
}