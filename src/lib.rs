use ahash::{AHashMap, AHashSet};
use std::path::PathBuf;

mod index;
mod init;
mod search;
mod write;

type Table = Vec<Row>;
type Row = Vec<String>;
struct SqlItems {
    indexs: AHashMap<String, Vec<u64>>,
    items: Vec<Table>,
    path: PathBuf,
}
impl SqlItems {
    pub fn new() {}
    pub fn init(path: impl Into<PathBuf>) -> Self {
        Self {
            indexs: (),
            items: (),
            path: path.into(),
        }
    }
    pub fn add() {}
    pub fn remove() {}
    pub fn insert() {}
}

struct NosqlItems {
    indexs: AHashMap<String, Vec<u64>>,
    items: Vec<Table>,
    path: PathBuf,
}
impl NosqlItems {
    pub fn new() {}
    pub fn init(path: impl Into<PathBuf>) -> Self {
        Self {
            indexs: (),
            items: (),
            path: path.into(),
        }
    }
    pub fn add() {}
    pub fn remove() {}
    pub fn insert() {}
}
