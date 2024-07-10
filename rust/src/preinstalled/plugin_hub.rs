use async_trait::async_trait;

pub enum PluginHubError {
    InstallationError,
}

#[derive(Debug)]
pub struct Plugin {
    pub name: String,
    pub github_url: String,
    pub description: String,
}

#[async_trait]
pub trait PluginsHubService {
    async fn get_all(&self) -> &Vec<Plugin>;
    async fn install(&self, plugins_dir: &str, plugin: &Plugin) -> Result<(), PluginHubError>;
}
