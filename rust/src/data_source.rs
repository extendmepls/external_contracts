use std::path::PathBuf;

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

#[derive(Debug)]
pub enum DataSourceError {
    NotFound,
    PermissionDenied,
    IoError(std::io::Error),
}

pub trait DataSource {
    fn watch(&self, on_changed: OnChangedCallback) -> Result<(), DataSourceError>;
    //TODO: return a mut/immutable reference to the data, to not stote it on the back and front on the same time
    fn open(&self, path: PathBuf) -> &FileNode;
}
