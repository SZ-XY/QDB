use ahash::RandomState;
use std::collections::HashMap;
use std::path::PathBuf;

mod index;
mod init;
mod new;
mod search;
mod write;

pub type IndexMap = HashMap<String, Vec<u64>, RandomState>;

//#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub type Table = Vec<Vec<String>>;

pub struct SqlItems {
    indexs: Vec<IndexMap>,
    items: Vec<Table>,
    path: PathBuf,
    sql_db_index: Vec<u64>,
}
impl SqlItems {
    pub fn add() {}
    pub fn remove() {}
    pub fn insert() {}
}

struct NosqlItems {
    indexs: Vec<IndexMap>,
    items: Vec<Table>,
    path: PathBuf,
    nosql_db_index: Vec<u64>,
}
impl NosqlItems {
    pub fn add() {}
    pub fn remove() {}
    pub fn insert() {}
}
