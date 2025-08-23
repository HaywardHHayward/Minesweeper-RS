use super::Message as SuperMessage;
use crate::{ArcLock, Config, Screen};

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug)]
pub struct Game {
    config: ArcLock<Config>,
}

impl Game {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self { config }
    }
}

impl Screen for Game {}
