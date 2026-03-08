use ahash::RandomState;
use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

mod index;
mod init;
mod search;
mod write;

pub type IndexMap = HashMap<String, Vec<u64>, RandomState>;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Table(Vec<Vec<String>>);

pub struct SqlItems {
    indexs: IndexMap,
    items: Vec<Table>,
    path: PathBuf,
}
impl SqlItems {
    pub fn new() {}
    pub fn add() {}
    pub fn remove() {}
    pub fn insert() {}
}

struct NosqlItems {
    indexs: IndexMap,
    items: Vec<Table>,
    path: PathBuf,
}
impl NosqlItems {
    pub fn new() {}
    pub fn add() {}
    pub fn remove() {}
    pub fn insert() {}
}
