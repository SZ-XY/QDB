use crate::new::*;
use crate::{IndexMap, NosqlItems, SqlItems, Table};
use rkyv::api::low::from_bytes;
use rkyv::rancor::Error;
use std::io::Read;
use std::{fs::File, path::PathBuf};

impl SqlItems {
    #[allow(dead_code)]
    pub fn init(path: impl Into<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.into();
        let db_index_path = path.join("sql_db_index");
        let sql_db_index = init_sql_db_index(db_index_path)?;
        Ok(Self {
            indexs: init_indexs(path.clone(), &sql_db_index)?,
            items: init_items(path.clone(), &sql_db_index)?,
            path,
            sql_db_index,
        })
    }
}
impl NosqlItems {
    #[allow(dead_code)]
    pub fn init(path: impl Into<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.into();
        let db_index_path = path.join("nosql_db_index");
        let nosql_db_index = init_nosql_db_index(db_index_path)?;
        Ok(Self {
            indexs: init_indexs(path.clone(), &nosql_db_index)?,
            items: init_items(path.clone(), &nosql_db_index)?,
            path,
            nosql_db_index,
        })
    }
}

fn init_items(
    path: PathBuf,
    db_index: &Vec<u64>,
) -> Result<Vec<Table>, Box<dyn std::error::Error>> {
    let mut item_list = Vec::new();
    for item in db_index {
        let file_path = path.join(item.to_string()).join("items");
        let items = init_item(file_path)?;
        item_list.push(items);
    }
    Ok(item_list)
}

fn init_item(path: PathBuf) -> Result<Table, Box<dyn std::error::Error>> {
    match File::open(&path) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            let items = from_bytes::<Table, Error>(&bytes[..])?;
            Ok(items)
            //access::<ArchivedVec<ArchivedTable>, Error>(&bytes[..]).unwrap()
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => new_item(path),
        Err(e) => Err(Box::new(e)),
    }
}
fn init_indexs(
    path: PathBuf,
    db_index: &Vec<u64>,
) -> Result<Vec<IndexMap>, Box<dyn std::error::Error>> {
    let mut index_list = Vec::new();
    for index in db_index {
        let file_path = path.join(index.to_string()).join("indexs");
        let index_map = init_index(file_path)?;
        index_list.push(index_map);
    }
    Ok(index_list)
}

fn init_index(path: PathBuf) -> Result<IndexMap, Box<dyn std::error::Error>> {
    match File::open(&path) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            let indexs = from_bytes::<IndexMap, Error>(&bytes[..])?;
            Ok(indexs)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => new_index(path),
        Err(e) => Err(Box::new(e)),
    }
}
fn init_sql_db_index(path: PathBuf) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    match File::open(&path) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            let items = from_bytes::<Vec<u64>, Error>(&bytes[..])?;
            Ok(items)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => new_sql_db_index(path),
        Err(e) => Err(Box::new(e)),
    }
}
fn init_nosql_db_index(path: PathBuf) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    match File::open(&path) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            let items = from_bytes::<Vec<u64>, Error>(&bytes[..])?;
            Ok(items)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => new_nosql_db_index(path),
        Err(e) => Err(Box::new(e)),
    }
}
