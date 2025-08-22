use crate::{Config, RcCell, Screen};

#[derive(Debug)]
pub enum Message {}

#[derive(Debug)]
pub struct MainMenu {
    config: RcCell<Config>,
}

impl MainMenu {
    pub fn build(config: RcCell<Config>) -> Self {
        Self { config }
    }
}

impl Screen for MainMenu {}
