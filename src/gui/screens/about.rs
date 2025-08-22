use crate::{Config, RcCell, Screen};

#[derive(Debug)]
pub enum Message {
    Back,
}

#[derive(Debug)]
pub struct About {
    config: RcCell<Config>,
}

impl Screen for About {}
