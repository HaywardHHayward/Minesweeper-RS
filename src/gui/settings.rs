use iced::{Element, Task, widget as GuiWidget};

use super::{Message as AppMessage, ScreenState, ScreenTrait};
pub struct Settings;

impl From<Settings> for ScreenState {
    fn from(settings: Settings) -> ScreenState {
        ScreenState::Settings(settings)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Exit,
}

impl From<Message> for AppMessage {
    fn from(value: Message) -> Self {
        AppMessage::Settings(value)
    }
}

impl ScreenTrait for Settings {
    type Message = Message;

    fn update(&mut self, message: Message) -> Task<AppMessage> {
        Task::none()
    }
    fn view(&self) -> Element<Self::Message> {
        GuiWidget::text("Hello").into()
    }
}
