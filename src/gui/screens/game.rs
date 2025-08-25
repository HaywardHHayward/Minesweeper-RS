use crate::{ArcLock, Config, Screen};

#[derive(Debug, Clone)]
pub enum Message {
    CreateSelf(u8, u8, u16),
}

#[derive(Debug)]
pub struct Game {
    config: ArcLock<Config>,
    board: crate::Board,
}

impl Game {
    pub fn build(config: ArcLock<Config>, board: crate::Board) -> Self {
        Self { config, board }
    }
}

impl Screen for Game {}
