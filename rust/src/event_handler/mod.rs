use async_trait::async_trait;

use crate::preinstalled::local_settings::PluginSettings;

pub enum EventError {
    ErrorPublishingEvent,
}

#[derive(Debug)]
pub enum Event {
    PluginSettingsChanged(PluginSettings),
}

#[async_trait]
pub trait EventBus {
    async fn publish(&self, event: Event) -> Result<(), EventError>;
}

#[async_trait]
pub trait EventListener {
    async fn listen(&mut self);
}
