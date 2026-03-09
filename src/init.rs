use crate::new::{new_index, new_item};
use crate::{IndexMap, NosqlItems, SqlItems, Table};
use rkyv::api::low::from_bytes;
use rkyv::rancor::Error;
use std::io::Read;
use std::{fs::File, path::PathBuf};

impl SqlItems {
    #[allow(dead_code)]
    pub fn init(path: impl Into<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.into();
        let item_path = path.join("items");
        let index_path = path.join("indexs");
        Ok(Self {
            indexs: init_indexs(index_path)?,
            items: init_items(item_path)?,
            path,
        })
    }
}
impl NosqlItems {
    #[allow(dead_code)]
    pub fn init(path: impl Into<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.into();
        let item_path = path.join("items");
        let index_path = path.join("indexs");
        Ok(Self {
            indexs: init_indexs(index_path)?,
            items: init_items(item_path)?,
            path,
        })
    }
}

fn init_items(path: PathBuf) -> Result<Vec<Table>, Box<dyn std::error::Error>> {
    match File::open(&path) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            let items = from_bytes::<Vec<Table>, Error>(&bytes[..])?;
            Ok(items)
            //access::<ArchivedVec<ArchivedTable>, Error>(&bytes[..]).unwrap()
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => new_item(path),
        Err(e) => Err(Box::new(e)),
    }
}
fn init_indexs(path: PathBuf) -> Result<IndexMap, Box<dyn std::error::Error>> {
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
