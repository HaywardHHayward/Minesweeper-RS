use crate::{Config, RcCell, Screen};

#[derive(Debug)]
pub enum Message {}

#[derive(Debug)]
pub struct GameSelection {
    config: RcCell<Config>,
}

impl Screen for GameSelection {}
