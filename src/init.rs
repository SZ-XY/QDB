use crate::new::{new_index, new_item};
use crate::{IndexMap, NosqlItems, SqlItems, Table};
use rkyv::api::low::from_bytes;
use rkyv::rancor::Error;
use std::io::Read;
use std::{fs::File, path::PathBuf};

impl SqlItems {
    #[allow(dead_code)]
    pub fn init(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let item_path = path.join("items");
        let index_path = path.join("indexs");
        Self {
            indexs: init_indexs(index_path),
            items: init_items(item_path),
            path,
        }
    }
}
impl NosqlItems {
    #[allow(dead_code)]
    pub fn init(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let item_path = path.join("items");
        let index_path = path.join("indexs");
        Self {
            indexs: init_indexs(index_path),
            items: init_items(item_path),
            path,
        }
    }
}

fn init_items(path: PathBuf) -> Vec<Table> {
    match File::open(&path) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).expect("Read items failed.");
            from_bytes::<Vec<Table>, Error>(&bytes[..]).unwrap()
            //access::<ArchivedVec<ArchivedTable>, Error>(&bytes[..]).unwrap()
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => new_item(path),
        Err(e) => panic!("Open items failed: {}", e),
    }
}
fn init_indexs(path: PathBuf) -> IndexMap {
    match File::open(&path) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).expect("Read indexs failed.");
            from_bytes::<IndexMap, Error>(&bytes[..]).unwrap()
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => new_index(path),
        Err(e) => panic!("Open indexs failed: {}", e),
    }
}
