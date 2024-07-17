use std::path::PathBuf;

pub type OnChangedCallback = fn();

#[derive(Debug)]
pub struct FileNode {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: u64,
    data: Option<Vec<u8>>,
    children: Vec<FileNode>,
}

pub enum DataSourceError {
    NotFound,
    PermissionDenied,
    IoError(std::io::Error),
}

pub trait DataSource {
    fn watch(&self, on_changed: OnChangedCallback) -> Result<(), DataSourceError>;
    //TODO: return a mut/immutable reference to the data, to not stote it on the back and front on the same time
    fn get_all_data(&self) -> Result<FileNode, DataSourceError>;
}
