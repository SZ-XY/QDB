use crate::{IndexMap, Table};
use rkyv::to_bytes;
use std::io::{BufWriter, Write};
use std::{collections::HashMap, fs::File, path::PathBuf};

pub fn new_item(path: PathBuf) -> Vec<Table> {
    let file = File::create(&path).expect("Create new item failed.");
    let new_item = Vec::new();
    let bytes = to_bytes::<rkyv::rancor::Error>(&new_item).expect("Serialization failed.");
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes).expect("Write failed.");
    writer.flush().expect("Flush failed.");
    new_item
}
pub fn new_index(path: PathBuf) -> IndexMap {
    let file = File::create(&path).expect("Create new item failed.");
    let new_index = HashMap::with_hasher(ahash::RandomState::default());
    let bytes = to_bytes::<rkyv::rancor::Error>(&new_index).expect("Serialization failed.");
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes).expect("Write failed.");
    writer.flush().expect("Flush failed.");
    new_index
}
