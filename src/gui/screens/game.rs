use crate::{Config, RcCell, Screen};

#[derive(Debug)]
pub enum Message {}

#[derive(Debug)]
pub struct Game {
    config: RcCell<Config>,
}

impl Screen for Game {}
