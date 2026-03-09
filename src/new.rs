use crate::{IndexMap, Table};
use rkyv::to_bytes;
use std::io::{BufWriter, Write};
use std::{collections::HashMap, fs::File, path::PathBuf};

pub fn new_item(path: PathBuf) -> Result<Vec<Table>, Box<dyn std::error::Error>> {
    let file = File::create(&path)?;
    let new_item = Vec::new();
    let bytes = to_bytes::<rkyv::rancor::Error>(&new_item)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes)?;
    writer.flush()?;
    Ok(new_item)
}
pub fn new_index(path: PathBuf) -> Result<IndexMap, Box<dyn std::error::Error>> {
    let file = File::create(&path)?;
    let new_index = HashMap::with_hasher(ahash::RandomState::default());
    let bytes = to_bytes::<rkyv::rancor::Error>(&new_index)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes)?;
    writer.flush()?;
    Ok(new_index)
}
