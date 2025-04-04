use iced::{Element, Task, widget as GuiWidget};

use super::Message as AppMessage;

pub struct MainMenu;

#[derive(Debug)]
pub enum Message {}

impl MainMenu {
    pub fn update(&mut self, message: Message) -> Task<AppMessage> {
        Task::none()
    }
    pub fn view(&self) -> Element<AppMessage> {
        let text = GuiWidget::Text::new("Main Menu");
        text.into()
    }
}
