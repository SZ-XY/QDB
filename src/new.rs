use crate::{IndexMap, Table};
use rkyv::api::low::from_bytes;
use rkyv::rancor::Error;
use rkyv::to_bytes;
use std::io::{BufWriter, Read, Write};
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
fn new_db_index(path: PathBuf, is_sql: bool) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let db_index_path = if is_sql {
        path.join("sql_db_index")
    } else {
        path.join("nosql_db_index")
    };
    let mut db_indexes = match File::open(&db_index_path) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            if bytes.is_empty() {
                Vec::new()
            } else {
                from_bytes::<Vec<u32>, Error>(&bytes)?
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(Box::new(e)),
    };
    let new_index = update_db_length(path)?;
    db_indexes.push(new_index);
    let bytes = to_bytes::<rkyv::rancor::Error>(&db_indexes)?;
    let file = File::create(&db_index_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes)?;
    writer.flush()?;
    Ok(db_indexes)
}
pub fn new_sql_db_index(path: PathBuf) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    new_db_index(path, true)
}
pub fn new_nosql_db_index(path: PathBuf) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    new_db_index(path, false)
}
fn update_db_length(path: PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
    let path = path.join("db_length");
    let db_index: u32 = match File::open(&path) {
        Ok(mut file) => {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            if contents.is_empty() {
                0
            } else {
                from_bytes::<u32, Error>(&contents)?
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => 0,
        Err(e) => return Err(Box::new(e)),
    };
    let next_db_index = db_index + 1;
    let bytes = to_bytes::<rkyv::rancor::Error>(&next_db_index)?;
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes)?;
    writer.flush()?;
    Ok(db_index)
}
