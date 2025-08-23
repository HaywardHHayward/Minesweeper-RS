use iced::Task;

use super::{AppMessage, MainMenu, Message as SuperMessage};
use crate::{ArcLock, Config, Screen};
#[derive(Debug, Clone)]
pub enum Message {
    Back,
}

#[derive(Debug)]
pub struct About {
    config: ArcLock<Config>,
}

impl About {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self { config }
    }
}

impl Screen for About {}
