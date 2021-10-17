use std::{collections::HashMap, path::PathBuf};

use crate::environment::Environment;

#[doc(hidden)]
#[derive(Debug, PartialEq)]
struct PoemConfig {
    pub active_env: Environment,
    config: HashMap<Environment, BasicConfig>,
}

#[derive(Debug)]
struct Database {
    pub(crate) adapter: String,
    pub(crate) db_name: String,
    pub(crate) pool: u32,
}

struct BasicConfig {
    pub enviroment: Environment,
    pub address: String,
    pub port: u16,
    pub database: Option<Database>,
    pub workers: Option<u16>,
    pub(crate) config_file_path: Option<PathBuf>,
    pub(crate) root_path: Option<PathBuf>,
}

impl BasicConfig {
    fn default() -> BasicConfig {
        Self {
            enviroment: todo!(),
            address: todo!(),
            port: todo!(),
            database: todo!(),
            workers: todo!(),
            config_file_path: todo!(),
            root_path: todo!(),
        }
    }
}