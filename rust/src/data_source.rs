use std::path::PathBuf;

use async_trait::async_trait;

pub type OnChangedCallback = fn();

#[derive(Debug)]
#[allow(dead_code)]
pub struct FileNode {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: u64,
    data: Result<Option<Vec<u8>>, DataSourceError>,
    children: Vec<FileNode>,
}

impl FileNode {
    pub fn new(name: String, path: PathBuf, is_dir: bool, size: u64) -> Self {
        Self {
            name,
            path,
            is_dir,
            size,
            data: Err(DataSourceError::NotFound),
            children: Vec::new(),
        }
    }

    pub fn with_data(mut self, data: Result<Option<Vec<u8>>, DataSourceError>) -> Self {
        self.data = data;
        self
    }

    pub fn with_children(mut self, children: Vec<FileNode>) -> Self {
        self.children = children;
        self
    }
}

#[derive(Debug)]
pub enum DataSourceError {
    NotFound,
    PermissionDenied,
    IoError(std::io::Error),
}

#[async_trait]
pub trait DataSource {
    fn watch(&self, on_changed: OnChangedCallback) -> Result<(), DataSourceError>;
    //TODO: return a mut/immutable reference to the data, to not stote it on the back and front on the same time
    async fn open(&self, path: PathBuf) -> Result<FileNode, DataSourceError>;
}
