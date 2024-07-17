#[derive(Debug)]
pub enum LocalSettingsError {
    UnableToLocateSettingsDir,
    UnableToCreateSettingsDir,
    UnableToSerializeData,
}

#[derive(Debug)]
pub struct PluginSettings {
    pub name: String,
    pub is_enabled: bool,
    pub path_to_shared_lib: String,
}

#[allow(dead_code)]
pub struct LocalSettings {
    plugins_dir: String,
    pub plugins: Vec<PluginSettings>,
}

pub trait LocalSettingsProvider {
    fn get_all_data(&self) -> LocalSettings;
    fn save(&self, data: String) -> Result<(), LocalSettingsError>;
}
