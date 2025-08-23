use super::Message as SuperMessage;
use crate::{ArcLock, Config, Screen};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
}

#[derive(Debug)]
pub struct SettingsScreen {
    config: ArcLock<Config>,
}

impl SettingsScreen {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self { config }
    }
}

impl Screen for SettingsScreen {}
