use crate::{IndexMap, Table};
use rkyv::to_bytes;
use std::io::{BufWriter, Write};
use std::{collections::HashMap, fs::File, path::PathBuf};

pub fn new_item(path: PathBuf) -> Result<Table, Box<dyn std::error::Error>> {
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
pub fn new_sql_db_index(path: PathBuf) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let file = File::create(&path)?;
    let new_sql_db_index = vec![0u64];
    let bytes = to_bytes::<rkyv::rancor::Error>(&new_sql_db_index)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes)?;
    writer.flush()?;
    Ok(new_sql_db_index)
}
pub fn new_nosql_db_index(path: PathBuf) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let file = File::create(&path)?;
    let new_nosql_db_index = Vec::new();
    let bytes = to_bytes::<rkyv::rancor::Error>(&new_nosql_db_index)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes)?;
    writer.flush()?;
    Ok(new_nosql_db_index)
}
